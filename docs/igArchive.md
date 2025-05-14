# igArchive Documentation (as implemented in ig-cauldron-rs)

# Archive Version 0x0C (latest?)
## Known games using this format:
- Crash N' sane trilogy.

# Archive Version 0x0B
## Known games using this format:
- Skylanders Imaginators
- Skylanders: Superchargers
- Skylanders: Trap Team
## Archive Header in order (size is 0x38)
- Table of Contents Size -> u32,
- File Count -> u32,
- Sector Size -> u32,
- Hash Search Divider -> u32,
- Hash Search Slop -> u32,
- Large File Block Count -> u32,
- Medium File Block Count -> u32,
- Small File Block Count -> u32,
- Name Table Offset -> u64,
- Name Table Size -> u32,
- Flags -> u32

# Archive Version 0x0A
## Known games using this format:
- Skylanders: Swap Force
- Skylanders: Lost Islands
## Archive Header in order (size is 0x38)
- Table of Contents Size -> u32,
- File Count -> u32,
- Sector Size -> u32,
- Hash Search Divider -> u32,
- Hash Search Slop -> u32,
- Large File Block Count -> u32,
- Medium File Block Count -> u32,
- Small File Block Count -> u32,
- Name Table Offset -> u64,
- Name Table Size -> u32,
- Flags -> u32

# Archive Version 0x08
## Known games using this format:
- Skylanders: Giants
- Skylanders Spyros Adventure (WiiU)
## Archive Header in order (size is 0x34)
- Table of Contents Size -> u32,
- File Count -> u32,
- Sector Size -> u32,
- Hash Search Divider -> u32,
- Hash Search Slop -> u32,
`in newer versions, the block counts come **before** the name table information`
- Name Table Offset -> **u32**, `in newer versions, this is a u64`
- Name Table Size -> u32,
- Large File Block Count -> u32,
- Medium File Block Count -> u32,
- Small File Block Count -> u32,
- Flags -> u32

# Archive Version 0x04
## Known games using this format:
- Skylanders Spyros Adventure (Wii)



# Misc Knowledge
- Tfb Tool games will always use LZMA compression. because they store all their data in level.bld, it makes sense to use this format. LZMA is really good at compressing massive chunks of data but has the downsides of significantly slower decompression times compared to Zlib and LZ4
