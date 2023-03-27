## run
## ```
## wapm install kranfix/string-utils --pip
## ```
from string_utils import bindings

su = bindings.string_utils()
texts = ["", "hello"]
for s in texts:
    print(f"'{s}': {su.is_empty(s)}")