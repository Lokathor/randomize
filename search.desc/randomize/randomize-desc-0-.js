searchState.loadedDescShard("randomize", 0, "Pseudo-random number generator crate.\nAllows sampling a <code>u16</code> number in <code>0 .. N</code>.\nAllows sampling a <code>u32</code> number in <code>0 .. N</code>.\nA trait for pseudo-random number generators with 32-bit …\nA Permuted Congruential Generator with 32-bit output.\nA Permuted Congruential Generator with 32-bit output, …\nThe number of possible outputs.\nThe number of possible outputs.\nGives a value in the range <code>1 ..= 10</code>\nGives a value in the range <code>1 ..= 12</code>\nGives a value in the range <code>1 ..= 20</code>\nGives a value in the range <code>1 ..= 4</code>\nGives a value in the range <code>1 ..= 6</code>\nGives a value in the range <code>1 ..= 8</code>\nThe generator’s extension array. The generator’s base …\nBase formulas used elsewhere in the crate.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreate a new generator seeded with data from getrandom.\nCreate a new generator seeded with data from getrandom.\nThe generator’s increment.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nJump the generator the given number of steps forward in …\nCreates a new generator by directly using the value given.\nCreates a new generator by directly using the value given.\nConstructs a new value.\nConstructs a new value.\nGives a uniformly distributed value.\nGives a value in the range <code>0.0 ..= 1.0</code>\nGives a uniformly distributed value.\nMakes the generator create the next output.\nGenerate the next <code>u32</code> in the sequence.\nGenerate the next <code>u32</code> in the sequence.\nGiven a <code>u32</code>, try to place it into this bounded range.\nGiven a <code>u16</code>, try to place it into this bounded range.\nGiven a generator function, call it until <code>place_in_range</code> …\nGiven a generator function, call it until <code>place_in_range</code> …\nRuns getrandom on the extension array.\nSeed a new generator.\nSeed a new generator.\nThe generator’s state.\nThe generator’s state.\nConstructs a new value, or <code>None</code> on failure.\nConstructs a new value, or <code>None</code> on failure.\nThis is the suggested multiplier for a PCG with 64 bits of …\nGenerates an <code>f32</code> in the signed or unsigned unit range.\nGenerates an <code>f64</code> in the signed or unsigned unit range.\nAdvance a 32-bit LCG by <code>delta</code> steps in <code>log2(delta)</code> time.\nAdvance a 32-bit LCG’s state.\nAdvance a 32-bit LCG by <code>delta</code> steps in <code>log2(delta)</code> time.\nAdvance a 32-bit LCG’s state.\nAdvance a 32-bit LCG by <code>delta</code> steps in <code>log2(delta)</code> time.\nAdvance a 32-bit LCG’s state.\nReturns <code>k</code> with probability <code>2^(-k-1)</code>, a “binary …\n“Xor-shift high bits” then “randomized rotate”, <code>u64</code>…\n“Xor-shift low bits” then “randomized rotate”, <code>u128</code>…")