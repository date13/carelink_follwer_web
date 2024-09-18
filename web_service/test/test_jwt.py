from core.auth import Auth
from models.beans import SysUser
from utils.tools import Tools

query = SysUser.select().where(SysUser.name == 'admin')
user = query[0]
auth = Auth()
# token = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE2OTA1MzAwNjMsImlhdCI6MTY5MDUzMDA2MCwic2NvcGUiOiJhY2Nlc3NfdG9rZW4iLCJzdWIiOiJ7XG4gICAgXCJhdmF0YXJcIjogbnVsbCxcbiAgICBcImdlbmRlclwiOiBudWxsLFxuICAgIFwiaWRcIjogODcsXG4gICAgXCJpbnNlcnRfdGltZVwiOiBcIjIwMjMtMDctMjdUMTY6MTg6MTRcIixcbiAgICBcImlzX2RlbGV0ZWRcIjogMCxcbiAgICBcIm5hbWVcIjogXCJhZG1pblwiLFxuICAgIFwicGFzc3dvcmRcIjogXCIzMDc4MGNjNmYyZTU2OTQ1YWFmOWM5NTc4YzkzMmUyMlwiLFxuICAgIFwicGhvbmVcIjogbnVsbCxcbiAgICBcInVwZGF0ZV90aW1lXCI6IG51bGxcbn0ifQ.X9QF_9fwVpk442laMLXw9yvLZQ_FDwNfl_ta8-BjvXs'
token = auth.encode_token(Tools.json(user.__data__))
print(token)
print(auth.decode_token(token))
password = auth.md5("admin")
print(password)
