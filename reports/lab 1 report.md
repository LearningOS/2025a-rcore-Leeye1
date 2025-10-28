# 实现的功能
不贴代码我难受, 可以选择不看
## Trace 0 & 1
将id视为指针,在对应地址写入或者读取1个字节的内容以isize数据类型返回
## Trace 2
1. 函数接口定义:我在os/src/task/mod.rs里面定义并且开放了add和read的接口,这个接口会调用TASK_MANAGER的增加与读取当前任务系统调用次数的方法然后返回. 在syscall分发前增加系统调用计数,在trace中读取计数
```
// os/src/syscall/mod.rs
syscall() -> add_current_count(id)  -> 调用 TASK_MANAGER.add_current_count(id)
// os/src/syscall/process.rs
sys_trace() -> read_current_count(id) -> 调用 TASK_MANAGER.read_current_count(id)
```
2. 结构体定义以及初始化:通过使用一个定长数组定义了一个记录所有syscall调用次数的数组; 其实我看到了ch3里有heap_alloc但是为了减少认知负担和实现难度于是就选择了用定长数组来解决,把负担放到kernel stack里
```rust
// os/src/task/mod.rs
pub struct TaskControlBlock {
/// The task status in it's lifecycle
pub task_status: TaskStatus,
/// The task context
pub task_cx: TaskContext,
/// the array of syscall count
pub task_syscall_count: [usize;MAX_SYSCALL_ID],
}

let mut tasks = [TaskControlBlock {
//..
	task_syscall_count:[0;MAX_SYSCALL_ID],
}; MAX_APP_NUM];
```
3. 函数实现: 函数通过访问TaskManagerInner从而实现通过对current task的系统调用id计数的增加与读取
```rust
fn add_current_count(&self,_id:usize){
	let mut inner = self.inner.exclusive_access();
	let current = inner.current_task;
	inner.tasks[current].task_syscall_count[_id]+=1;
	drop(inner);
}

fn read_current_count(&self,_id:usize) -> usize{
	let inner = self.inner.exclusive_access();
	let current = inner.current_task;
	inner.tasks[current].task_syscall_count[_id]
}
```
# 问答题
1. ==正确进入 U 态后，程序的==特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 [三个 bad 测例 (ch2b_bad_*.rs)](https://github.com/LearningOS/rCore-Tutorial-Test-2025S/tree/master/src/bin) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。
    
2. 深入理解 [trap.S](https://github.com/LearningOS/rCore-Tutorial-Code-2025S/blob/ch3/os/src/trap/trap.S) 中两个函数 `__alltraps` 和 `__restore` 的作用，并回答如下问题:
    
    1. L40：刚进入 `__restore` 时，`sp` 代表了什么值。请指出 `__restore` 的两种使用情景: 刚进入`__restore`时,sp是kernel_stack的地址,目的是为了恢复Task的TrapContext; 应用场景1是从Task调用系统调用陷入Trap然后通过`__restore`返回到应用, 应用场景2是Task之间切换应用后通过`__switch`切换TaskContext然后跳转到`__restore` next task的TrapContext
        
    2. L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释: 前面3行将kernel stack中trapcontext里的三个系统级别寄存器(sstatus, sepc, sscratch)放到了临时寄存器; 第一个scrw恢复 sstatus（确保 SPP=U 等），为 sret 做好状态准备,在执行 `sret` 时，硬件会根据 `sstatus.SPP` 决定跳回 U 模式;第二个,恢复 sepc(sret 后 PC 将跳到这个寄存器保存的地址);第三个是回复用户栈指针

        
        ld t0, 32*8(sp)
        ld t1, 33*8(sp)
        ld t2, 2*8(sp) 
        csrw sstatus, t0 
        csrw sepc, t1
        csrw sscratch, t2
        
    3. L50-L56：为何跳过了 `x2` 和 `x4`: x2是sp寄存器, x4是tp寄存器(用户态不需要使用这个寄存器);这两个寄存器在前面all_trap的时候就没有被保存,所以跳过了这两个寄存器
        
        ld x1, 1*8(sp)
        ld x3, 3*8(sp)
        .set n, 5
        .rept 27
           LOAD_GP %n
           .set n, n+1
        .endr
        
    4. L60：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？:这里的sp是内核栈的指针,然后放到sscratch中用于临时保存这个任务的内核栈地址,然后sscratch在all_trap中保存了用户台的指针,所以这样用户栈就可以恢复了
        
        csrrw sp, sscratch, sp
        
    5. `__restore`：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？:sret, 这里会直接跳转到sepc寄存器中的地址也就是 Trap 发生之前执行的最后一条指令的地址
        
    6. L13：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？:将用户栈保存到sscratch,将内核栈保存到sp寄存器
        
        csrrw sp, sscratch, sp
        
    7. 从 U 态进入 S 态是哪一条指令发生的？:是在ecall的时候发生的

# 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
```
chatgpt: 详细情况可以到 chap 3 作业中查看
	1. 解读实验任务的意思和解惑(没有让gpt生成任何代码)
	2. 询问如何引用其他模块函数(pub,mod)
	3. 解决rust语法层面上的bug(如何定义数组,缺少注释)
```
1. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

```
loader::get_num_app()
loader::load_app()
task::TaskManager.run_next_task()(主要参考了如何访问inner的TCB数据结构)
```
3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。
4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。