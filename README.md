# adventofcode

So far, this is 100% in rust.

This repo follows the [automation guidelines](https://www.reddit.com/r/adventofcode/wiki/faqs/automation) on the [r/adventofcode](https://reddit.com/r/adventofcode) community in these ways:

1. Inputs are cached locally - in each rust project as input.txt.
2. The User-Agent header is set as me

See [common/src/lib.rs - download_input](https://github.com/jessehansen/adventofcode/blob/main/common/src/lib.rs#L67) for implementation

NOTE: I only run this once per day, so I don't worry about throttling. If you use this app and run multiple days at once, please insert a sleep between each run so it doesn't hammer AoC :).
