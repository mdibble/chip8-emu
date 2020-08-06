# CHIP-8 Emulator in Rust
Emulator of the CHIP-8 instruction set. Nearly all of the opcodes are included, but there are a select few that haven't been written.

# Installation
The front-end was done with <code>SDL2</code>, so you'll need to have that installed to be able to run the emulator. Once it is installed, you can
just clone the repository and run <code>cargo run</code>, which should compile and run the program.

# Usage
Different ROMs can be loaded by editing the <code>path</code> variable in <code>main.rs</code>. Not the most user-friendly method, but it works. I've
included a couple basic ROMs that I've verified work as intended, but feel free to test them out with other <code>.ch8</code> ROMs.

# To-do
<ul>
  <li>Add audio beeps</li>
  <li>Add remaining opcodes</li>
  <li>Test more ROMs</li>
  <li>Allow ROMs to be added using command-line arguments</li>
</ul>

# Resources
The main resources I used were the <a href='https://en.wikipedia.org/wiki/CHIP-8'>CHIP-8 Wikipedia Article</a> and
<a href='http://devernay.free.fr/hacks/chip8/C8TECH10.HTM'>This Reference</a> on the instruction set.
