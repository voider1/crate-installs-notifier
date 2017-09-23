# crate-installs-notifier
A little program which shows you the amount of downloads of you crate.
Only works on MacOS.

## Setup
Firstly, run it once with the release flag (this makes sure the config file is generated):

```shell
cargo run --release
```

After this has completed you'll notice nothing has happened, but if you look at `~/.config/crate-installs-notifier/config.yaml` you can configure the program.
Add an entry to the YAML array with the attributes name: string and downloads: int. If you run it again it'll show you a notification and update the downloads attribute.

Now you should customize the plist file in the repo to your liking
(refer to [this page](https://developer.apple.com/library/content/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/ScheduledJobs.html#//apple_ref/doc/uid/10000172i-CH1-SW2))
for more information on how to customize the config.

When you're done customizing, copy the plist file to `~/Library/LaunchAgents`, then load the plist file:

```shell
launchctl load ~/Library/LaunchAgents/com.voider1.crate-installs-notifier.plist
```

The program will run once to show you it has succesfully loaded and then it will run it each time you've configured it to run.
