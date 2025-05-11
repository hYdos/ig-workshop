# ig-workshop
(another) all in one solution for editing Alchemy Games. This tool will be a work in progress for a while as far as I know, and is just an experiment I'm trying out and an excuse to rewrite/semi-port igLibrary for my other projects. If you use this tool at the moment I will not provide support it is **nowhere near done**, though suggestions are welcome :)

# Core Goals
- User-friendliness. The UI should be intuitive and use simple easy to understand language wherever possible.
- Support for every main line game. Though, I will not guarantee the order I work on these (SSF will definitely be near the end)
- No Static State. This means multiple games can be open and edited at the exact same time
- undo/redo tracking. Always copy on write to any igObject's in the game storing the changes in a Queue that can be popped off at any time
- transfer igObject api. There will be a minimal version of this present in the core already, but having an api for others to work off allows any object to be transferred automatically with a one of cost of effort.
- extension api. having a `plugins` folder which allows rust cdylib libraries to load custom functionality. Examples: custom ui for igObject or custom igObject transfer (see above)
- keeping ig-library, ig-workshop, and ig-extensions separate. This keeps the code clean and helps illustrate where one part might need more functions to help the other
- keep match/switch statements as simple/easy to read as possible. Instead of using strings or integers, Where possible use enums. This also helps cover versioning differences for example with igz fixups
- deep copy (copy all children) and surface copy (keep references). This is one area where igCauldron seems to really have issues. This will allow for quicker modding because you will have that base to work off

## Secondary Goals
- store version info in bottom right for better user bug reporting
- search bar in ui to allow for better discovery of files/objects
- igx and igb support
- wav, vvl's, etc asset exporting

## Far Future Goals
- Ability to write a new game from scratch

# Harass me to add these (Or PR them for me 🙏)
- creating a new game from scratch
- as close as possible to being able to load and then save a file and have it match 1:1 with the input on both Tfb Games and VV Games

# Credits:
- Nefarious Tech Support for all the work on Reverse Engineering and igRewrites 6(skydra),7, and 8(igCauldron/igLibrary)
- LG For General alchemy RE
- the general SRE Community