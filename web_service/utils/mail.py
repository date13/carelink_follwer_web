import zmail

from core.config import config


class MailObject(object):

    def __init__(self):
        # 邮箱账号
        self.username = config.MAIL_USER

        # 邮箱授权码
        self.authorization_code = config.MAIL_PWD

        # 构建一个邮箱服务对象
        self.server = zmail.server(self.username, self.authorization_code, smtp_host=config.MAIL_SMTP_HOST)
        # self.server.smtp_server = "smtp.exmail.qq.com"

    def send(self, mailObj):
        self.server.send_mail(config.MAIL_TO, mailObj)
