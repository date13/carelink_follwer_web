from pydantic import (BaseModel, ValidationError, field_validator)
from pydantic_core.core_schema import FieldValidationInfo

from utils.tools import Tools


class UserModel(BaseModel):
    id: int
    name: str

    @field_validator('name')
    @classmethod
    def name_must_contain_space(cls, v: str) -> str:
        if v is None or v == '':
            raise ValueError('姓名不得为空')
        return v.title()

    # you can select multiple fields, or use '*' to select all fields
    @field_validator('id', 'name')
    @classmethod
    def check_alphanumeric(cls, v: str, info: FieldValidationInfo) -> str:
        if isinstance(v, str):
            # info.field_name is the name of the field being validated
            is_alphanumeric = v.replace(' ', '').isalnum()
            assert is_alphanumeric, f'{info.field_name} must be alphanumeric'
        return v


try:
    print(UserModel().model_validate_json())
except ValidationError as e:
    print(Tools.parseValidation(e.errors()))
    print(e.errors())
# > id=1 name='John Doe'

try:
    UserModel(id=1, name='samuel')
except ValidationError as e:
    print(e.errors())
    """
    1 validation error for UserModel
    name
      Value error, must contain a space [type=value_error, input_value='samuel', input_type=str]
    """

try:
    UserModel(id='abc', name='John Doe')
except ValidationError as e:
    print(e.errors())
    """
    1 validation error for UserModel
    id
      Input should be a valid integer, unable to parse string as an integer [type=int_parsing, input_value='abc', input_type=str]
    """

try:
    UserModel(id=1, name='John Doe!')
except ValidationError as e:
    print(e.errors())
    """
    1 validation error for UserModel
    name
      Assertion failed, name must be alphanumeric
    assert False [type=assertion_error, input_value='John Doe!', input_type=str]
    """
