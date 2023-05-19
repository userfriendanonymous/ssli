# About
Claw is CLI tool for interacting with Scratch. (such as following users or loving a project)

# Installation
Currently only possible through
```
cargo install ssli
```
But this is not very accessible, I will add other methods asap... :/

# Credentials
Your entered Scratch credentials are stored at:

- Linux: `/home/alice/.local/share/ssli`
- macOS: `/Users/Alice/Library/Application Support/org.UserFriend.ssli`
- Windows: `C:\Users\Alice\AppData\Roaming\UserFriend\ssli\data`

You can always clear all ssli's data through:
```
ssli reset
```

# Login
```
ssli login [NAME]
```

`[NAME]` - name for the session, for example 'main' (this is not name of your Scratch account!)
- Example:
```
ssli login main
  - or..
ssli login alt
```
After entering command it will ask for credentials (username and password).


# Cookie based login
```
ssli auth [NAME]
```
After entering command asks for your Scratch cookies.

# Manage auth sessions
To switch current account to another
```
ssli switch [NAME]
```
For example
```
ssli switch alt
  - or..
ssli switch main
```

# Removing auth session
```
ssli unauth [NAME]
```

# Usage examples
```
ssli user griffpatch fol    // follow griffpatch
ssli studio 114 lock        // make studio private
ssli studio 114 tgc         // toggle studio comments
ssli studio 114 title "Cool studio"     // set studio title
ssli project 114            // get and output project general metadata
```

# Help
```
ssli -h
ssli user --help
ssli studio -h
```

# Note
There is a problem that it says operation was successful while it actually wasn't. So not everything saying "Success" was actually done, don't get scared.
