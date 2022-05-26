# Console Command Client
 
ConsoleSocketSample1 is control Imagenics machine.
 
## Installation

- Download a compiled binary for Windows [releases](https://github.com/akiraoku/cccr/releases/download/v1/cccr.exe) page.
- Download a compiled binary for Linux [releases](https://github.com/akiraoku/cccr/releases/download/v1/cccr) page.

## Usage

1. Syntax

```
cccr cccr  [tcp|udp] ['Commnad'] [addr:port]
```

* argument "udp" is not yet implemented as of v1.x .

2. Example1 - Send Single Command -

```
cccr tcp "#$eA00000" 192.168.2.254:1300
```

3. Example2 - Combining Commands -

```
cccr tcp "1,1\r2,2\r3,3\rw" 192.168.2.254:1300
```

4. Example3 - Redirecting to log.txt -

```
cccr cccr tcp "w" 192.168.2.254:1300 >> log.txt
```


## Contributing
 
1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin my-new-feature`
5. Submit a pull request :D

## History
 
Version 1.01.0
 
## Credits
 
Lead Developer - Oku,Akira
