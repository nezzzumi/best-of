## Best Of

This tool runs a command/program 10 times and shows the min, max and average time elapsed (like Linux's [time](https://man7.org/linux/man-pages/man1/time.1.html)).

> The runs are synchronous and single-threaded.

### Installation

Just download the [latest release](https://github.com/nezzzumi/best-of/releases/latest).

### Usage

You need to run the tool passing the command/program as an argument.

> The default number of runs is 10, but you can change it in the source code.

Example:

```bash
$ ./best-of ls -la

the min was 0.001849301s
the max was 0.009776543s
the avg was 0.0028993331s
```

### References

The idea came when I was watching [Antony's](https://www.twitch.tv/anthonywritescode) live stream, and he was using something similar.
