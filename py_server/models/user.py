from pydantic import BaseModel


class User(BaseModel):
    id: str
    full_name: str 
    team: str 
    creation_date: str
    active: int
    admin: int
    