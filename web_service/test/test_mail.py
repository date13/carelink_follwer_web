from utils.mail import MailObject

mailBody = {
    'subject': 'carelink_follower_web警报',
    'content_text': '测试邮件',  # 纯文本或者HTML内容
}
mailObj = MailObject()
mailObj.send(mailBody)
