# Smash Board 

Imagine stack based editor which is under the hood just clipboard which you now and love.

#### Tasks / TODO

- Figure out how to spawn Daemom to handle sqlite database ?
  - Will probably need to make library 
- The compose will open interactive window
  - A rewrite is pending
- Also need to make sure the only string are allowed in the db
  - Very difficult problem; Can be enforced by many tricks: 
   - Like limit the length of string 
   - Take clipboards help to inforce it
- Will need environment agnostic integration with clipboard
- Add commands like: 
  - Clear History
