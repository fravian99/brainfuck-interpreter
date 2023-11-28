# Brainfuck

## Introduction

### Instructions
It contains the instructions.

```brainfuck
> = increases memory pointer, or moves the pointer to the right 1 block
< = decreases memory pointer, or moves the pointer to the left 1 block
+ = increases value stored at the block pointed to by the memory pointer
- = decreases value stored at the block pointed to by the memory pointer
. = output the byte at the data pointer as a character.
, = input a character and store it in the byte at the data pointer.
[ = if the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command
] =If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.
```

### Memory

It is a struct that contains a vector of blocks of memory and a pointer to a position of the memory.


### Handler
Runs the instructions recived.

### Parser
Converts the code to instructions.
