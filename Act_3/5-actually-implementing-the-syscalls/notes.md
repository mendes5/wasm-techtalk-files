45 and 192 will be called
brk and mmap respectively.

For growing the program memory allocation:
45 = https://man7.org/linux/man-pages/man2/brk.2.html 

Which we will not be implementing since it would only be a `return 0`.

For mapping the memory alocated memory: 
192 = https://man7.org/linux/man-pages/man2/mmap2.2.html
      https://man7.org/linux/man-pages/man2/mmap.2.html


Those numbers will not match the original linux syscalls values and idk why...
Maybe due to the custom libc implementation
