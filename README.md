# lekker
## progress log:
13 Nov 2026: i suddenly have a will to do this and i just start building it and currently done the lexer(labeling) part,left over the parser and the execution

14 Nov 2026:i done the parser init and brain already kinda tired,time to rest bruh

15 Nov 2026:i done the parser second(also can call merging part) and actually need 30 min but i used 1 hour and 30 min in total because i used 1 hour to write useless comments holly cow i really suck bruh get a job ...
## Features lekker will have:
1. you can use function name as variables,like print or input,it wont affect calling print and input 
2. everytime entering a loop you will loss access to outer loop local variables,so if you need access, set it outside all loops so it will be a global variable and pull it with @
3. you cannot name variables that contains any numbers and dot
4. in print lekker dont use comma(,) to separate,you dont need comma,just simply use "" as separate between variables and variables,also it wont auto jump to next line so remember to add "\n" (the print will always flush first)
5. lekker use ^ and ~ to represent True and False
6. ( as break and ) as continue
7. in print you use "" to separate,dont use , it wont work on lekker, "," is only for list item separation and also attribute separation
8. if need wrap then inner use ? instead of ; example:
```lekker
  !if|!equal{@x,@y}?|{!loop{blablabla...}?};
  !else{!loop{blablabla..}?};
```
