# ScapyCOBOLBankLab

Custom ZOS mainframe system simulator,  
That transfers packets that are vulnerable to command injection if you use scapy CPY and RUN will be subcommands
Rust backend simulating a filepath manipulation and a command injection vulnerability 


## outline

rust based
`justfile` for running targets
`SCAPY-README.md` gives you some pointers

### ./client/*

client will ask server to transfer bank balance datafiles over TCP like:

```
(headers)
TYPE    |SUBTYPE |TO    |FROM   |NOTE1           |NOTE2           |

(datafiles over tcp, 1 line=1 tcp transmission)
BANKDATA|XFER    |JACKY |BECKY  |0001000X        |X               |
BANKDATA|XFER    |BECKY |JACKY  |0000500X        |X               |
BANKCMD |CPY     |X     |X      |/tmp/foo.txt    |/tmp/foo2.txt   |
BANKCMD |RUN     |X     |X      |/tmp/a.sh arg1  |                |
```

### ./server/*

server will process client commands and dumbly execute them

## goal

your goal is to use scapy to inject a reverse shell or something similar.
