# Qudc

This is a simple parser that converts a qudit-based quantum circuit into an equivalent directed acyclic graph.

For example, consider a simple program consisting of Hadamard gates and CNOT gates applied to a qutrit (3-dimensional) and a qubit (2-dimensional).

```
qudit 0 3
qudit 1 2

h 0
h 1

c 0 1

h 0

m 0
m 1
```

The resulting graph:

![](examples/example.png)

Or as a `dot` file:

```dot
digraph {
    0 [ label = "3" ]
    1 [ label = "2" ]
    2 [ label = "3" ]
    3 [ label = "2" ]
    4 [ label = "3" ]
    5 [ label = "2" ]
    6 [ label = "3" ]
    0 -> 2 [ label = "'H'" ]
    1 -> 3 [ label = "'H'" ]
    2 -> 4 [ label = "'I'" ]
    3 -> 5 [ label = "'I'" ]
    2 -> 5 [ label = "'I'" ]
    4 -> 6 [ label = "'H'" ]
}
```

## Usage

`cargo run filename`