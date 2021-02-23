import enum

from string_sum import Value, PyEnum
import os;


os.environ['RUST_BACKTRACE'] = '1'


class Cats(enum.Enum):
    black = 1
    white = 2
    red = 3


# raise error: TypeError: Attempted to reuse key: 'black'
class Snakes(enum.Enum):
    black = 1
    # black = 2


# raise error: AttributeError: Cannot reassign members
# Cats.black = 7
# Cats.black.value = 7

# raise error: ValueError: 6 is not a valid Cats -- Cats(6)

class _New(PyEnum):

    def __getattr__(self, item):
        return self.get_member(item)


class _Enum(type):

    def __new__(cls, name, bases, namespace):
        return _New(name, namespace)


class NewENum(metaclass=_Enum):
    black = 1
    white = 2


class NewENumClone(metaclass=_Enum):
    black = 1
    white = 2


assert NewENum.black.name == 'black'
assert NewENum.black.value == 1
assert NewENum.black == Cats.black
assert NewENum.black != Cats.white
assert NewENum.black < Cats.white
assert NewENum.white > Cats.black
assert NewENum.white >= Cats.black
assert NewENum.black <= Cats.white
assert (NewENum.white > 1) is False  # Python Cats will raised

assert str(NewENum.black) == 'NewENum.black'
assert hasattr(NewENum.black, '__hash__')
assert hasattr(NewENum, '__iter__')
assert hasattr(NewENum, '__len__')
assert hasattr(NewENum, '__contains__')
assert hasattr(NewENum, '__call__')
assert hasattr(NewENum, '__getitem__')
# assert NewENum(NewENum.black)
# assert {NewENum.black: '12312'}
# assert NewENum(6)

print(
    NewENum(NewENumClone.black),  # broken!
    NewENumClone.black in NewENum,  # broken!
    # NewENum.__reversed__,
    # Cats(NewENumClone.black)
    Cats.black == Snakes.black  # False
)


# NewENum.black = 1  # broken!
# NewENum.black.value = 1

class Enum:
    map_values: dict
    map_keys: dict

    _hash: str
    _classname: str
