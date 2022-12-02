import reichard
def on_tick(*args, **kwargs):
    print("on_tick")
    doug = reichard.Doug(23)
    print(doug)
    print(reichard.double(2))
def on_init(*args, **kwargs):
    print("on_init")
    doug = reichard.Doug(23)
    print(doug)
    print(reichard.double(2))

