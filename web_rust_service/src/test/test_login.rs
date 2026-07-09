use regex::Regex;
use reqwest::cookie::Jar;
use reqwest::{redirect::Policy, Client, ClientBuilder};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

#[derive(Debug)]
pub struct CareLinkLogin {
    client: Client,
    base_url: String,
    country: String,
    language: String,
}

#[allow(dead_code)]
#[allow(unused)]
impl CareLinkLogin {
    pub fn new(country: &str, language: &str) -> Result<Self, Box<dyn Error>> {
        // 创建cookie jar
        let cookie_jar = Jar::default();

        // 创建HTTP客户端，禁用自动重定向以便手动处理
        let client = ClientBuilder::new()
            .cookie_provider(Arc::new(cookie_jar))
            .redirect(Policy::none())  // 禁用自动重定向
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .build()?;

        Ok(Self {
            client,
            base_url: "https://carelink.minimed.eu".to_string(),
            country: country.to_string(),
            language: language.to_string(),
        })
    }

    async fn extract_location_header(&self, response: &reqwest::Response) -> Option<String> {
        response.headers()
            .get("location")
            .and_then(|value| value.to_str().ok())
            .map(|s| s.to_string())
    }

    async fn extract_cookies(&self, response: &reqwest::Response) -> HashMap<String, String> {
        let mut cookies = HashMap::new();
        for cookie in response.cookies() {
            cookies.insert(cookie.name().to_string(), cookie.value().to_string());
        }
        cookies
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<String, Box<dyn Error>> {
        println!("Step 1: Starting login process...");

        // Step 1: 访问初始地址获取授权地址
        let initial_url = format!(
            "{}/patient/sso/login?country={}&lang={}",
            self.base_url, self.country, self.language
        );

        println!("访问: {}", initial_url);
        let response = self.client.get(&initial_url).send().await?;
        println!("Status: {}", response.status());

        let location = self.extract_location_header(&response).await
            .ok_or("No location header in step 1")?;
        println!("获取到location: {}", location);

        // Step 2: 访问授权地址
        println!("\nStep 2: 访问授权地址");
        let response = self.client.get(&location).send().await?;
        println!("Status: {}", response.status());

        let location2 = self.extract_location_header(&response).await
            .ok_or("No location header in step 2")?;

        // 处理相对URL
        let final_login_url = if location2.starts_with("/") {
            format!("https://carelink-login.minimed.eu{}", location2)
        } else {
            location2
        };
        println!("最终登录地址: {}", final_login_url);

        // Step 3: 访问登录页面获取必要参数
        println!("\nStep 3: 访问登录页面");
        let response = self.client.get(&final_login_url).send().await?;
        println!("Status: {}", response.status());

        // 提取state参数
        let state_regex = Regex::new(r"state=([^&]+)")?;
        let state = state_regex.captures(&final_login_url)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .ok_or("无法提取state参数")?;
        println!("提取到state参数: {}", state);

        // 检查当前cookie
        let cookies = self.extract_cookies(&response).await;
        println!("当前cookies: {:?}", cookies);

        // Step 4: 提交登录表单
        println!("\nStep 4: 提交登录信息");

        let mut form_data = HashMap::new();
        form_data.insert("state", state);
        form_data.insert("username", username);
        form_data.insert("password", password);
        form_data.insert("action", "default");

        println!("提交表单数据: {:?}", form_data);

        let response = self.client.post(&final_login_url)
            .form(&form_data)
            .send()
            .await?;

        println!("登录响应状态: {}", response.status());
        let status_code = response.status();
        if status_code.is_success() || status_code.is_redirection() {
            let location3 = self.extract_location_header(&response).await
                .ok_or("登录后没有location header")?;

            println!("登录后重定向到: {}", location3);

            // 处理相对URL
            let resume_url = if location3.starts_with("/") {
                format!("https://carelink-login.minimed.eu{}", location3)
            } else {
                location3
            };

            println!("Resume URL: {}", resume_url);

            // Step 5: 访问resume地址
            println!("\nStep 5: 访问resume地址");
            let response = self.client.get(&resume_url).send().await?;
            println!("Status: {}", response.status());

            let location4 = self.extract_location_header(&response).await
                .ok_or("resume后没有location header")?;
            println!("获取到location: {}", location4);

            // Step 6: 访问最终授权地址
            println!("\nStep 6: 访问最终授权地址");
            let response = self.client.get(&location4).send().await?;

            let status = response.status().as_u16();
            println!("Status: {}", status);

            // 处理307重定向
            if status == 307 {
                // 获取Set-Cookie头部
                let cookies = response.headers()
                    .get_all("set-cookie")
                    .iter()
                    .filter_map(|value| value.to_str().ok())
                    .collect::<Vec<_>>();

                println!("Set-Cookies: {:?}", cookies);

                // 查找auth_tmp_token
                for cookie in cookies {
                    if cookie.contains("auth_tmp_token") {
                        let token_regex = Regex::new(r"auth_tmp_token=([^;]+)")?;
                        if let Some(caps) = token_regex.captures(cookie) {
                            if let Some(token_match) = caps.get(1) {
                                let token = token_match.as_str();
                                println!("\n✅ 成功获取到auth_tmp_token!");
                                return Ok(token.to_string());
                            }
                        }
                    }
                }

                // 如果没有在Set-Cookie中找到，检查响应体
                let body = &response.text().await?;
                println!("响应体: {}", body);
                // 尝试从响应体中提取
                let token_regex = Regex::new(r"auth_tmp_token=([^;]+)")?;
                if let Some(caps) = token_regex.captures(&body) {
                    if let Some(token_match) = caps.get(1) {
                        let token = token_match.as_str();
                        println!("\n✅ 成功获取到auth_tmp_token!");
                        return Ok(token.to_string());
                    }
                }

                Err("无法找到auth_tmp_token".into())
            } else {
                // 如果不是307，检查响应
                let body = response.text().await?;
                println!("响应体: {}", body);
                Err(format!("预期307重定向，但得到状态码: {}", status).into())
            }
        } else {
            let error_body = response.text().await?;
            println!("登录失败，响应体: {}", error_body);
            Err(format!("登录失败，状态码: {}", status_code).into())
        }
    }
}
#[allow(dead_code)]
#[allow(unused)]
pub async fn test_carelink_login() -> Result<(), Box<dyn Error>> {
    println!("CareLink 自动登录程序");
    println!("=====================\n");

    // 创建登录实例
    let login_client = CareLinkLogin::new("hk", "zh")?;

    // 测试账号密码 - 请替换为真实的账号密码
    let username = "date13";
    let password = "pigMing1983!";

    // 执行登录
    match login_client.login(username, password).await {
        Ok(token) => {
            println!("\n🎉 登录成功！");
            println!("auth_tmp_token: {}", token);

            // 可以在这里将token保存到文件或数据库
            // std::fs::write("auth_token.txt", &token)?;
            // println!("Token已保存到 auth_token.txt");
        }
        Err(e) => {
            println!("\n❌ 登录失败: {}", e);
        }
    }

    Ok(())
}
