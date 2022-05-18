from dataclasses import dataclass, field
import random
import string
from unicodedata import name

def generate_id() -> str:
    return "".join(random.choices(string.ascii_uppercase,k=12))
#if dataclass(frozen=True) only readable
#if kw_only= True => initialize using only myclass(name="nithin") not myclass("nithin")
@dataclass
class PersonData:
    name:str
    address:str
    active:bool = True
    emailId:list[str]= field(default_factory=list) # default factory pointer problem 
    personId: str = field(init=False,default_factory=generate_id) #cant call at init
    _search_string: str = field(init=False)
    _uniq_comment: str =field(init=False,repr=False)

    def __post_init__(self)->None:
        self._search_string=f"{self.name} {self.address[:3]}"
        self._uniq_comment="Tthis will not be displayed"
class Person:
    def __init__(self,name: str, address: str) -> None:
        self.name=name
        self.address=address

    def __str__(self) -> str:
        return f"name: {self.name} \naddress:{self.address}"

def main() -> None:
    person = Person(name="john",address="123 main streeet")
    print(person)
    personD=PersonData(name="John",address="6525 adfs street")
    print(personD)
    print(personD.name)
if __name__=="__main__":
    main()