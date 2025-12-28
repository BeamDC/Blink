# Blink
the blink programming language

# Future Ideas
## Optimization
 - Partial Application for non-constant functions at compile time
   - flag all const and non-const args for a function call
   - inline all the constant args, and perform folding to try and minimize instructions
   - if the resulting code is an improvement over the original, we generate a function pointer to a new instruction set taking only the non constant args
   - replace the original call with a call to the new function.
     - In theory this can lead to big optimizations if large sections of the function only use compile time arguments
