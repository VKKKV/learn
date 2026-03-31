from pwn import *

context.terminal = ["tmux", "splitw", "-h"]
context.arch = "amd64"

p = gdb.debug(
    "./main",
    env={"SHELL": "/bin/bash"},
    gdbscript="""
     break *vulnerable_function+71
              continue
""",
)

p.send(cyclic(400))

p.interactive()
