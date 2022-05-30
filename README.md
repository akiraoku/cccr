# Console Command Client by Rust

Console Command Client can control IMAGENICS machine via commandlines.

## Installation

- Download a compiled binary for Windows [releases](https://github.com/akiraoku/cccr/releases/download/v1/cccr.exe) page.
- Download a compiled binary for Linux [releases](https://github.com/akiraoku/cccr/releases/download/v1/cccr) page.
- Download a compiled binary for macOS [releases](https://github.com/akiraoku/cccr/releases/download/v1/for_intel_mac_cccr.zip) page.

## Usage

1. Syntax

```
cccr [tcp|udp] ["commnad"] [addr:port]
```

* 'command' must be enclosed in double quotes.
* The argument "udp" is not yet implemented as of v1.x .
* The most trailing carriage return can be reduced.

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
cccr tcp "w" 192.168.2.254:1300 >> ./log.txt
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
