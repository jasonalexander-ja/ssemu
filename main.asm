ldn $start_value  ; Loads 10 into the accumulator 

:loop_start_value ; The memory address the loop should return to 
sub $subtract_val ; Subtract 1 from the accumulator 
cmp               ; Skip the next jump instruction if the accumulator is negative 
jmp $loop_start   ; Jump to the start of the loop 
stp               ; Program stops when the accumulator is negative 

:loop_start       ; Pointer to the memory address the loop should return to 
abs $loop_start_value

:subtract_val     ; Value to be subtracted
abs 0d1

:start_value      ; Value to start in the accumulator 
abs 0d-10