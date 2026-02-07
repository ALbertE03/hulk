; HULK -> LLVM IR  (generated)
declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)
declare i8* @malloc(i64)
declare i8* @realloc(i8*, i64)
declare void @free(i8*)
declare i64 @strlen(i8*)
declare i8* @strcpy(i8*, i8*)
declare i8* @strcat(i8*, i8*)
declare i32 @snprintf(i8*, i64, i8*, ...)
declare i32 @rand()
declare void @srand(i32)
declare i64 @time(i64*)
declare void @abort()
declare double @llvm.pow.f64(double, double)
declare double @llvm.sin.f64(double)
declare double @llvm.cos.f64(double)
declare double @llvm.exp.f64(double)
declare double @llvm.log.f64(double)
declare double @llvm.sqrt.f64(double)
declare double @llvm.fabs.f64(double)

@.fmt_num  = private unnamed_addr constant [5 x i8] c"%.6g\00"
@.fmt_str  = private unnamed_addr constant [3 x i8] c"%s\00"
@.fmt_nl   = private unnamed_addr constant [2 x i8] c"\0A\00"
@.true_s   = private unnamed_addr constant [5 x i8] c"true\00"
@.false_s  = private unnamed_addr constant [6 x i8] c"false\00"
@.space_s  = private unnamed_addr constant [2 x i8] c" \00"
@.empty_s  = private unnamed_addr constant [1 x i8] c"\00"
@.oob_msg  = private unnamed_addr constant [36 x i8] c"Runtime error: index out of bounds\0A\00"
@.rand_seeded = global i1 false

; GC tracking: a growable array of i8* pointers
@.gc_buf   = global i8** null
@.gc_len   = global i64 0
@.gc_cap   = global i64 0

%T.Greeter = type { i64, double }
@.slit_16 = private unnamed_addr constant [3 x i8] c", \00"
@.slit_50 = private unnamed_addr constant [2 x i8] c"!\00"
%T.Adder = type { i64, double }
%T.Animal = type { i64, double, double }
@.slit_95 = private unnamed_addr constant [6 x i8] c"I am \00"
@.slit_122 = private unnamed_addr constant [7 x i8] c" with \00"
@.slit_161 = private unnamed_addr constant [6 x i8] c" legs\00"
%T.Mammal = type { i64, double, double, double }
@.slit_222 = private unnamed_addr constant [13 x i8] c"warm-blooded\00"
@.slit_225 = private unnamed_addr constant [13 x i8] c"cold-blooded\00"
%T.Cat = type { i64, double, double, double, double }
@.slit_257 = private unnamed_addr constant [13 x i8] c" says: Meow!\00"
@.slit_298 = private unnamed_addr constant [14 x i8] c" (indoor cat)\00"
@.slit_325 = private unnamed_addr constant [15 x i8] c" (outdoor cat)\00"
%T.Doubler = type { i64 }
%T.Dog = type { i64, double, double, double, double }
@.slit_378 = private unnamed_addr constant [13 x i8] c" says: Woof!\00"
@.slit_405 = private unnamed_addr constant [6 x i8] c" the \00"
%T.Bird = type { i64, double, double, double }
@.slit_486 = private unnamed_addr constant [10 x i8] c" can fly!\00"
@.slit_513 = private unnamed_addr constant [12 x i8] c" cannot fly\00"
@.slit_532 = private unnamed_addr constant [97 x i8] c"────────────────────────────────\00"
@.slit_590 = private unnamed_addr constant [1 x i8] c"\00"
@.slit_623 = private unnamed_addr constant [30 x i8] c"=== ZOO MANAGEMENT SYSTEM ===\00"
@.slit_631 = private unnamed_addr constant [4 x i8] c"Rex\00"
@.slit_634 = private unnamed_addr constant [9 x i8] c"Labrador\00"
@.slit_641 = private unnamed_addr constant [5 x i8] c"Milo\00"
@.slit_644 = private unnamed_addr constant [7 x i8] c"Beagle\00"
@.slit_651 = private unnamed_addr constant [5 x i8] c"Luna\00"
@.slit_658 = private unnamed_addr constant [6 x i8] c"Simba\00"
@.slit_665 = private unnamed_addr constant [7 x i8] c"Tweety\00"
@.slit_672 = private unnamed_addr constant [5 x i8] c"Kiwi\00"
@.slit_679 = private unnamed_addr constant [25 x i8] c"--- Animal Greetings ---\00"
@.slit_717 = private unnamed_addr constant [20 x i8] c"--- Dog Actions ---\00"
@.slit_765 = private unnamed_addr constant [20 x i8] c"--- Cat Actions ---\00"
@.slit_813 = private unnamed_addr constant [18 x i8] c"--- Bird Info ---\00"
@.slit_841 = private unnamed_addr constant [26 x i8] c"--- Mammal Properties ---\00"
@.slit_869 = private unnamed_addr constant [25 x i8] c"--- Type Checks (is) ---\00"
@.slit_876 = private unnamed_addr constant [15 x i8] c"Rex is Animal?\00"
@.slit_908 = private unnamed_addr constant [15 x i8] c"Rex is Mammal?\00"
@.slit_934 = private unnamed_addr constant [12 x i8] c"Rex is Dog?\00"
@.slit_954 = private unnamed_addr constant [18 x i8] c"Tweety is Mammal?\00"
@.slit_980 = private unnamed_addr constant [18 x i8] c"Tweety is Animal?\00"
@.slit_1013 = private unnamed_addr constant [29 x i8] c"--- Zoo Census (Vectors) ---\00"
@.slit_1055 = private unnamed_addr constant [19 x i8] c"Total legs in zoo:\00"
@.slit_1103 = private unnamed_addr constant [20 x i8] c"Doubled leg counts:\00"
@.slit_1132 = private unnamed_addr constant [31 x i8] c"--- Lambda & Closure Tests ---\00"
@.slit_1152 = private unnamed_addr constant [12 x i8] c"5 squared =\00"
@.slit_1201 = private unnamed_addr constant [17 x i8] c"10 scaled by 3 =\00"
@.slit_1237 = private unnamed_addr constant [16 x i8] c"7 scaled by 3 =\00"
@.slit_1275 = private unnamed_addr constant [8 x i8] c"Hello, \00"
@.slit_1294 = private unnamed_addr constant [2 x i8] c"!\00"
@.slit_1320 = private unnamed_addr constant [6 x i8] c"World\00"
@.slit_1344 = private unnamed_addr constant [5 x i8] c"HULK\00"
@.slit_1369 = private unnamed_addr constant [23 x i8] c"--- Math Functions ---\00"
@.slit_1376 = private unnamed_addr constant [5 x i8] c"PI =\00"
@.slit_1395 = private unnamed_addr constant [5 x i8] c"E  =\00"
@.slit_1414 = private unnamed_addr constant [12 x i8] c"sqrt(144) =\00"
@.slit_1433 = private unnamed_addr constant [9 x i8] c"sin(0) =\00"
@.slit_1452 = private unnamed_addr constant [9 x i8] c"cos(0) =\00"
@.slit_1471 = private unnamed_addr constant [9 x i8] c"exp(1) =\00"
@.slit_1490 = private unnamed_addr constant [16 x i8] c"log(10, 1000) =\00"
@.slit_1518 = private unnamed_addr constant [14 x i8] c"Random [0,1):\00"
@.slit_1527 = private unnamed_addr constant [31 x i8] c"--- Fibonacci (while loop) ---\00"
@.slit_1555 = private unnamed_addr constant [26 x i8] c"--- Variable Mutation ---\00"
@.slit_1571 = private unnamed_addr constant [10 x i8] c"counter =\00"
@.slit_1596 = private unnamed_addr constant [16 x i8] c"Final counter =\00"
@.slit_1617 = private unnamed_addr constant [26 x i8] c"--- String Operations ---\00"
@.slit_1624 = private unnamed_addr constant [5 x i8] c"HULK\00"
@.slit_1628 = private unnamed_addr constant [9 x i8] c"Compiler\00"
@.slit_1632 = private unnamed_addr constant [4 x i8] c"2.0\00"
@.slit_1636 = private unnamed_addr constant [19 x i8] c"HULK Compiler v2.0\00"
@.slit_1643 = private unnamed_addr constant [18 x i8] c"HULK Compiler 2.0\00"
@.slit_1650 = private unnamed_addr constant [2 x i8] c"*\00"
@.slit_1656 = private unnamed_addr constant [2 x i8] c" \00"
@.slit_1674 = private unnamed_addr constant [5 x i8] c"HULK\00"
@.slit_1692 = private unnamed_addr constant [2 x i8] c" \00"
@.slit_1731 = private unnamed_addr constant [26 x i8] c"--- Utility Functions ---\00"
@.slit_1738 = private unnamed_addr constant [11 x i8] c"abs(-42) =\00"
@.slit_1758 = private unnamed_addr constant [14 x i8] c"max(10, 20) =\00"
@.slit_1778 = private unnamed_addr constant [14 x i8] c"min(10, 20) =\00"
@.slit_1798 = private unnamed_addr constant [21 x i8] c"clamp(150, 0, 100) =\00"
@.slit_1818 = private unnamed_addr constant [20 x i8] c"clamp(-5, 0, 100) =\00"
@.slit_1839 = private unnamed_addr constant [22 x i8] c"--- Is Quadruped? ---\00"
@.slit_1846 = private unnamed_addr constant [14 x i8] c"Rex (4 legs):\00"
@.slit_1861 = private unnamed_addr constant [17 x i8] c"Tweety (2 legs):\00"
@.slit_1877 = private unnamed_addr constant [21 x i8] c"--- Macros (def) ---\00"
@.slit_1884 = private unnamed_addr constant [12 x i8] c"double(7) =\00"
@.slit_1903 = private unnamed_addr constant [12 x i8] c"triple(5) =\00"
@.slit_1922 = private unnamed_addr constant [20 x i8] c"double(triple(3)) =\00"
@.slit_1941 = private unnamed_addr constant [13 x i8] c"negate(42) =\00"
@.slit_1960 = private unnamed_addr constant [13 x i8] c"Hola, Mundo!\00"
@.slit_1967 = private unnamed_addr constant [12 x i8] c"Hola, HULK!\00"
@.slit_1974 = private unnamed_addr constant [17 x i8] c"is_positive(42):\00"
@.slit_1983 = private unnamed_addr constant [17 x i8] c"is_positive(-5):\00"
@.slit_1993 = private unnamed_addr constant [26 x i8] c"--- Functors (invoke) ---\00"
@.slit_2004 = private unnamed_addr constant [14 x i8] c"Doubler(10) =\00"
@.slit_2029 = private unnamed_addr constant [14 x i8] c"Doubler(25) =\00"
@.slit_2058 = private unnamed_addr constant [15 x i8] c"Adder(5)(10) =\00"
@.slit_2083 = private unnamed_addr constant [16 x i8] c"Adder(5)(100) =\00"
@.slit_2108 = private unnamed_addr constant [6 x i8] c"Hello\00"
@.slit_2120 = private unnamed_addr constant [6 x i8] c"World\00"
@.slit_2133 = private unnamed_addr constant [5 x i8] c"HULK\00"
@.slit_2142 = private unnamed_addr constant [25 x i8] c"--- Protocol Extends ---\00"
@.slit_2149 = private unnamed_addr constant [21 x i8] c"Describable objects:\00"
@.slit_2187 = private unnamed_addr constant [27 x i8] c"=== ALL TESTS COMPLETE ===\00"

define i8* @Greeter_new(double %prefix) {
entry:
  %t0 = call i8* @malloc(i64 16)
  call void @__hulk_gc_track(i8* %t0)
  %t1 = bitcast i8* %t0 to %T.Greeter*
  %t2 = getelementptr inbounds %T.Greeter, %T.Greeter* %t1, i32 0, i32 0
  store i64 1, i64* %t2
  %t3 = alloca double
  store double %prefix, double* %t3
  %t4 = load double, double* %t3
  %t5 = getelementptr inbounds %T.Greeter, %T.Greeter* %t1, i32 0, i32 1
  store double %t4, double* %t5
  ret i8* %t0
}

define double @Greeter_invoke(i8* %self, double %name) {
entry:
  %t6 = alloca double
  store double %name, double* %t6
  %t7 = ptrtoint i8* %self to i64
  %t8 = bitcast i64 %t7 to double
  %t9 = bitcast double %t8 to i64
  %t10 = alloca i64
  store i64 %t9, i64* %t10
  %t11 = load i64, i64* %t10
  %t12 = inttoptr i64 %t11 to i8*
  %t13 = bitcast i8* %t12 to %T.Greeter*
  %t14 = getelementptr inbounds %T.Greeter, %T.Greeter* %t13, i32 0, i32 1
  %t15 = load double, double* %t14
  %t17 = ptrtoint i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.slit_16, i64 0, i64 0) to i64
  %t18 = bitcast i64 %t17 to double
  %t20 = bitcast double %t15 to i64
  %t21 = alloca i64
  store i64 %t20, i64* %t21
  %t22 = load i64, i64* %t21
  %t23 = inttoptr i64 %t22 to i8*
  %t24 = bitcast double %t18 to i64
  %t25 = alloca i64
  store i64 %t24, i64* %t25
  %t26 = load i64, i64* %t25
  %t27 = inttoptr i64 %t26 to i8*
  %t28 = call i64 @strlen(i8* %t23)
  %t29 = call i64 @strlen(i8* %t27)
  %t30 = add i64 %t28, %t29
  %t31 = add i64 %t30, 1
  %t32 = call i8* @malloc(i64 %t31)
  call void @__hulk_gc_track(i8* %t32)
  call i8* @strcpy(i8* %t32, i8* %t23)
  call i8* @strcat(i8* %t32, i8* %t27)
  %t33 = ptrtoint i8* %t32 to i64
  %t19 = bitcast i64 %t33 to double
  %t34 = load double, double* %t6
  %t36 = bitcast double %t19 to i64
  %t37 = alloca i64
  store i64 %t36, i64* %t37
  %t38 = load i64, i64* %t37
  %t39 = inttoptr i64 %t38 to i8*
  %t40 = bitcast double %t34 to i64
  %t41 = alloca i64
  store i64 %t40, i64* %t41
  %t42 = load i64, i64* %t41
  %t43 = inttoptr i64 %t42 to i8*
  %t44 = call i64 @strlen(i8* %t39)
  %t45 = call i64 @strlen(i8* %t43)
  %t46 = add i64 %t44, %t45
  %t47 = add i64 %t46, 1
  %t48 = call i8* @malloc(i64 %t47)
  call void @__hulk_gc_track(i8* %t48)
  call i8* @strcpy(i8* %t48, i8* %t39)
  call i8* @strcat(i8* %t48, i8* %t43)
  %t49 = ptrtoint i8* %t48 to i64
  %t35 = bitcast i64 %t49 to double
  %t51 = ptrtoint i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.slit_50, i64 0, i64 0) to i64
  %t52 = bitcast i64 %t51 to double
  %t54 = bitcast double %t35 to i64
  %t55 = alloca i64
  store i64 %t54, i64* %t55
  %t56 = load i64, i64* %t55
  %t57 = inttoptr i64 %t56 to i8*
  %t58 = bitcast double %t52 to i64
  %t59 = alloca i64
  store i64 %t58, i64* %t59
  %t60 = load i64, i64* %t59
  %t61 = inttoptr i64 %t60 to i8*
  %t62 = call i64 @strlen(i8* %t57)
  %t63 = call i64 @strlen(i8* %t61)
  %t64 = add i64 %t62, %t63
  %t65 = add i64 %t64, 1
  %t66 = call i8* @malloc(i64 %t65)
  call void @__hulk_gc_track(i8* %t66)
  call i8* @strcpy(i8* %t66, i8* %t57)
  call i8* @strcat(i8* %t66, i8* %t61)
  %t67 = ptrtoint i8* %t66 to i64
  %t53 = bitcast i64 %t67 to double
  ret double %t53
}

define i8* @Adder_new(double %offset) {
entry:
  %t68 = call i8* @malloc(i64 16)
  call void @__hulk_gc_track(i8* %t68)
  %t69 = bitcast i8* %t68 to %T.Adder*
  %t70 = getelementptr inbounds %T.Adder, %T.Adder* %t69, i32 0, i32 0
  store i64 2, i64* %t70
  %t71 = alloca double
  store double %offset, double* %t71
  %t72 = load double, double* %t71
  %t73 = getelementptr inbounds %T.Adder, %T.Adder* %t69, i32 0, i32 1
  store double %t72, double* %t73
  ret i8* %t68
}

define double @Adder_invoke(i8* %self, double %x) {
entry:
  %t74 = alloca double
  store double %x, double* %t74
  %t75 = load double, double* %t74
  %t76 = ptrtoint i8* %self to i64
  %t77 = bitcast i64 %t76 to double
  %t78 = bitcast double %t77 to i64
  %t79 = alloca i64
  store i64 %t78, i64* %t79
  %t80 = load i64, i64* %t79
  %t81 = inttoptr i64 %t80 to i8*
  %t82 = bitcast i8* %t81 to %T.Adder*
  %t83 = getelementptr inbounds %T.Adder, %T.Adder* %t82, i32 0, i32 1
  %t84 = load double, double* %t83
  %t85 = fadd double %t75, %t84
  ret double %t85
}

define i8* @Animal_new(double %name, double %legs) {
entry:
  %t86 = call i8* @malloc(i64 24)
  call void @__hulk_gc_track(i8* %t86)
  %t87 = bitcast i8* %t86 to %T.Animal*
  %t88 = getelementptr inbounds %T.Animal, %T.Animal* %t87, i32 0, i32 0
  store i64 3, i64* %t88
  %t89 = alloca double
  store double %name, double* %t89
  %t90 = alloca double
  store double %legs, double* %t90
  %t91 = load double, double* %t89
  %t92 = getelementptr inbounds %T.Animal, %T.Animal* %t87, i32 0, i32 1
  store double %t91, double* %t92
  %t93 = load double, double* %t90
  %t94 = getelementptr inbounds %T.Animal, %T.Animal* %t87, i32 0, i32 2
  store double %t93, double* %t94
  ret i8* %t86
}

define double @Animal_greet(i8* %self) {
entry:
  %t96 = ptrtoint i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.slit_95, i64 0, i64 0) to i64
  %t97 = bitcast i64 %t96 to double
  %t98 = ptrtoint i8* %self to i64
  %t99 = bitcast i64 %t98 to double
  %t100 = bitcast double %t99 to i64
  %t101 = alloca i64
  store i64 %t100, i64* %t101
  %t102 = load i64, i64* %t101
  %t103 = inttoptr i64 %t102 to i8*
  %t104 = bitcast i8* %t103 to %T.Animal*
  %t105 = getelementptr inbounds %T.Animal, %T.Animal* %t104, i32 0, i32 1
  %t106 = load double, double* %t105
  %t108 = bitcast double %t97 to i64
  %t109 = alloca i64
  store i64 %t108, i64* %t109
  %t110 = load i64, i64* %t109
  %t111 = inttoptr i64 %t110 to i8*
  %t112 = bitcast double %t106 to i64
  %t113 = alloca i64
  store i64 %t112, i64* %t113
  %t114 = load i64, i64* %t113
  %t115 = inttoptr i64 %t114 to i8*
  %t116 = call i64 @strlen(i8* %t111)
  %t117 = call i64 @strlen(i8* %t115)
  %t118 = add i64 %t116, %t117
  %t119 = add i64 %t118, 1
  %t120 = call i8* @malloc(i64 %t119)
  call void @__hulk_gc_track(i8* %t120)
  call i8* @strcpy(i8* %t120, i8* %t111)
  call i8* @strcat(i8* %t120, i8* %t115)
  %t121 = ptrtoint i8* %t120 to i64
  %t107 = bitcast i64 %t121 to double
  %t123 = ptrtoint i8* getelementptr inbounds ([7 x i8], [7 x i8]* @.slit_122, i64 0, i64 0) to i64
  %t124 = bitcast i64 %t123 to double
  %t126 = bitcast double %t107 to i64
  %t127 = alloca i64
  store i64 %t126, i64* %t127
  %t128 = load i64, i64* %t127
  %t129 = inttoptr i64 %t128 to i8*
  %t130 = bitcast double %t124 to i64
  %t131 = alloca i64
  store i64 %t130, i64* %t131
  %t132 = load i64, i64* %t131
  %t133 = inttoptr i64 %t132 to i8*
  %t134 = call i64 @strlen(i8* %t129)
  %t135 = call i64 @strlen(i8* %t133)
  %t136 = add i64 %t134, %t135
  %t137 = add i64 %t136, 1
  %t138 = call i8* @malloc(i64 %t137)
  call void @__hulk_gc_track(i8* %t138)
  call i8* @strcpy(i8* %t138, i8* %t129)
  call i8* @strcat(i8* %t138, i8* %t133)
  %t139 = ptrtoint i8* %t138 to i64
  %t125 = bitcast i64 %t139 to double
  %t140 = ptrtoint i8* %self to i64
  %t141 = bitcast i64 %t140 to double
  %t142 = bitcast double %t141 to i64
  %t143 = alloca i64
  store i64 %t142, i64* %t143
  %t144 = load i64, i64* %t143
  %t145 = inttoptr i64 %t144 to i8*
  %t146 = bitcast i8* %t145 to %T.Animal*
  %t147 = getelementptr inbounds %T.Animal, %T.Animal* %t146, i32 0, i32 2
  %t148 = load double, double* %t147
  %t150 = bitcast double %t125 to i64
  %t151 = alloca i64
  store i64 %t150, i64* %t151
  %t152 = load i64, i64* %t151
  %t153 = inttoptr i64 %t152 to i8*
  %t154 = call i8* @__hulk_num_to_str(double %t148)
  %t155 = call i64 @strlen(i8* %t153)
  %t156 = call i64 @strlen(i8* %t154)
  %t157 = add i64 %t155, %t156
  %t158 = add i64 %t157, 2
  %t159 = call i8* @malloc(i64 %t158)
  call void @__hulk_gc_track(i8* %t159)
  call i8* @strcpy(i8* %t159, i8* %t153)
  call i8* @strcat(i8* %t159, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t159, i8* %t154)
  %t160 = ptrtoint i8* %t159 to i64
  %t149 = bitcast i64 %t160 to double
  %t162 = ptrtoint i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.slit_161, i64 0, i64 0) to i64
  %t163 = bitcast i64 %t162 to double
  %t165 = bitcast double %t149 to i64
  %t166 = alloca i64
  store i64 %t165, i64* %t166
  %t167 = load i64, i64* %t166
  %t168 = inttoptr i64 %t167 to i8*
  %t169 = bitcast double %t163 to i64
  %t170 = alloca i64
  store i64 %t169, i64* %t170
  %t171 = load i64, i64* %t170
  %t172 = inttoptr i64 %t171 to i8*
  %t173 = call i64 @strlen(i8* %t168)
  %t174 = call i64 @strlen(i8* %t172)
  %t175 = add i64 %t173, %t174
  %t176 = add i64 %t175, 2
  %t177 = call i8* @malloc(i64 %t176)
  call void @__hulk_gc_track(i8* %t177)
  call i8* @strcpy(i8* %t177, i8* %t168)
  call i8* @strcat(i8* %t177, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t177, i8* %t172)
  %t178 = ptrtoint i8* %t177 to i64
  %t164 = bitcast i64 %t178 to double
  ret double %t164
}

define double @Animal_is_quadruped(i8* %self) {
entry:
  %t179 = ptrtoint i8* %self to i64
  %t180 = bitcast i64 %t179 to double
  %t181 = bitcast double %t180 to i64
  %t182 = alloca i64
  store i64 %t181, i64* %t182
  %t183 = load i64, i64* %t182
  %t184 = inttoptr i64 %t183 to i8*
  %t185 = bitcast i8* %t184 to %T.Animal*
  %t186 = getelementptr inbounds %T.Animal, %T.Animal* %t185, i32 0, i32 2
  %t187 = load double, double* %t186
  %t189 = fcmp oeq double %t187, 4.0e0
  %t188 = uitofp i1 %t189 to double
  ret double %t188
}

define i8* @Mammal_new(double %name, double %legs, double %warm) {
entry:
  %t190 = call i8* @malloc(i64 32)
  call void @__hulk_gc_track(i8* %t190)
  %t191 = bitcast i8* %t190 to %T.Mammal*
  %t192 = getelementptr inbounds %T.Mammal, %T.Mammal* %t191, i32 0, i32 0
  store i64 4, i64* %t192
  %t193 = alloca double
  store double %name, double* %t193
  %t194 = alloca double
  store double %legs, double* %t194
  %t195 = alloca double
  store double %warm, double* %t195
  %t196 = load double, double* %t193
  %t197 = load double, double* %t194
  %t198 = call i8* @Animal_new(double %t196, double %t197)
  %t199 = bitcast i8* %t198 to %T.Animal*
  %t200 = getelementptr inbounds %T.Animal, %T.Animal* %t199, i32 0, i32 1
  %t201 = load double, double* %t200
  %t202 = getelementptr inbounds %T.Mammal, %T.Mammal* %t191, i32 0, i32 1
  store double %t201, double* %t202
  %t203 = getelementptr inbounds %T.Animal, %T.Animal* %t199, i32 0, i32 2
  %t204 = load double, double* %t203
  %t205 = getelementptr inbounds %T.Mammal, %T.Mammal* %t191, i32 0, i32 2
  store double %t204, double* %t205
  %t206 = load double, double* %t195
  %t207 = getelementptr inbounds %T.Mammal, %T.Mammal* %t191, i32 0, i32 3
  store double %t206, double* %t207
  ret i8* %t190
}

define double @Mammal_body_temp(i8* %self) {
entry:
  %t208 = ptrtoint i8* %self to i64
  %t209 = bitcast i64 %t208 to double
  %t210 = bitcast double %t209 to i64
  %t211 = alloca i64
  store i64 %t210, i64* %t211
  %t212 = load i64, i64* %t211
  %t213 = inttoptr i64 %t212 to i8*
  %t214 = bitcast i8* %t213 to %T.Mammal*
  %t215 = getelementptr inbounds %T.Mammal, %T.Mammal* %t214, i32 0, i32 3
  %t216 = load double, double* %t215
  %t217 = fcmp one double %t216, 0.0
  %t218 = alloca double
  br i1 %t217, label %then_219, label %else_220
then_219:
  %t223 = ptrtoint i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.slit_222, i64 0, i64 0) to i64
  %t224 = bitcast i64 %t223 to double
  store double %t224, double* %t218
  br label %merge_221
else_220:
  %t226 = ptrtoint i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.slit_225, i64 0, i64 0) to i64
  %t227 = bitcast i64 %t226 to double
  store double %t227, double* %t218
  br label %merge_221
merge_221:
  %t228 = load double, double* %t218
  ret double %t228
}

define i8* @Cat_new(double %name, double %indoor) {
entry:
  %t229 = call i8* @malloc(i64 40)
  call void @__hulk_gc_track(i8* %t229)
  %t230 = bitcast i8* %t229 to %T.Cat*
  %t231 = getelementptr inbounds %T.Cat, %T.Cat* %t230, i32 0, i32 0
  store i64 5, i64* %t231
  %t232 = alloca double
  store double %name, double* %t232
  %t233 = alloca double
  store double %indoor, double* %t233
  %t234 = load double, double* %t232
  %t235 = call i8* @Mammal_new(double %t234, double 4.0e0, double 1.0)
  %t236 = bitcast i8* %t235 to %T.Mammal*
  %t237 = getelementptr inbounds %T.Mammal, %T.Mammal* %t236, i32 0, i32 1
  %t238 = load double, double* %t237
  %t239 = getelementptr inbounds %T.Cat, %T.Cat* %t230, i32 0, i32 1
  store double %t238, double* %t239
  %t240 = getelementptr inbounds %T.Mammal, %T.Mammal* %t236, i32 0, i32 2
  %t241 = load double, double* %t240
  %t242 = getelementptr inbounds %T.Cat, %T.Cat* %t230, i32 0, i32 2
  store double %t241, double* %t242
  %t243 = getelementptr inbounds %T.Mammal, %T.Mammal* %t236, i32 0, i32 3
  %t244 = load double, double* %t243
  %t245 = getelementptr inbounds %T.Cat, %T.Cat* %t230, i32 0, i32 3
  store double %t244, double* %t245
  %t246 = load double, double* %t233
  %t247 = getelementptr inbounds %T.Cat, %T.Cat* %t230, i32 0, i32 4
  store double %t246, double* %t247
  ret i8* %t229
}

define double @Cat_meow(i8* %self) {
entry:
  %t248 = ptrtoint i8* %self to i64
  %t249 = bitcast i64 %t248 to double
  %t250 = bitcast double %t249 to i64
  %t251 = alloca i64
  store i64 %t250, i64* %t251
  %t252 = load i64, i64* %t251
  %t253 = inttoptr i64 %t252 to i8*
  %t254 = bitcast i8* %t253 to %T.Cat*
  %t255 = getelementptr inbounds %T.Cat, %T.Cat* %t254, i32 0, i32 1
  %t256 = load double, double* %t255
  %t258 = ptrtoint i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.slit_257, i64 0, i64 0) to i64
  %t259 = bitcast i64 %t258 to double
  %t261 = bitcast double %t256 to i64
  %t262 = alloca i64
  store i64 %t261, i64* %t262
  %t263 = load i64, i64* %t262
  %t264 = inttoptr i64 %t263 to i8*
  %t265 = bitcast double %t259 to i64
  %t266 = alloca i64
  store i64 %t265, i64* %t266
  %t267 = load i64, i64* %t266
  %t268 = inttoptr i64 %t267 to i8*
  %t269 = call i64 @strlen(i8* %t264)
  %t270 = call i64 @strlen(i8* %t268)
  %t271 = add i64 %t269, %t270
  %t272 = add i64 %t271, 1
  %t273 = call i8* @malloc(i64 %t272)
  call void @__hulk_gc_track(i8* %t273)
  call i8* @strcpy(i8* %t273, i8* %t264)
  call i8* @strcat(i8* %t273, i8* %t268)
  %t274 = ptrtoint i8* %t273 to i64
  %t260 = bitcast i64 %t274 to double
  ret double %t260
}

define double @Cat_describe(i8* %self) {
entry:
  %t275 = ptrtoint i8* %self to i64
  %t276 = bitcast i64 %t275 to double
  %t277 = bitcast double %t276 to i64
  %t278 = alloca i64
  store i64 %t277, i64* %t278
  %t279 = load i64, i64* %t278
  %t280 = inttoptr i64 %t279 to i8*
  %t281 = bitcast i8* %t280 to %T.Cat*
  %t282 = getelementptr inbounds %T.Cat, %T.Cat* %t281, i32 0, i32 4
  %t283 = load double, double* %t282
  %t284 = fcmp one double %t283, 0.0
  %t285 = alloca double
  br i1 %t284, label %then_286, label %else_287
then_286:
  %t289 = ptrtoint i8* %self to i64
  %t290 = bitcast i64 %t289 to double
  %t291 = bitcast double %t290 to i64
  %t292 = alloca i64
  store i64 %t291, i64* %t292
  %t293 = load i64, i64* %t292
  %t294 = inttoptr i64 %t293 to i8*
  %t295 = bitcast i8* %t294 to %T.Cat*
  %t296 = getelementptr inbounds %T.Cat, %T.Cat* %t295, i32 0, i32 1
  %t297 = load double, double* %t296
  %t299 = ptrtoint i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.slit_298, i64 0, i64 0) to i64
  %t300 = bitcast i64 %t299 to double
  %t302 = bitcast double %t297 to i64
  %t303 = alloca i64
  store i64 %t302, i64* %t303
  %t304 = load i64, i64* %t303
  %t305 = inttoptr i64 %t304 to i8*
  %t306 = bitcast double %t300 to i64
  %t307 = alloca i64
  store i64 %t306, i64* %t307
  %t308 = load i64, i64* %t307
  %t309 = inttoptr i64 %t308 to i8*
  %t310 = call i64 @strlen(i8* %t305)
  %t311 = call i64 @strlen(i8* %t309)
  %t312 = add i64 %t310, %t311
  %t313 = add i64 %t312, 1
  %t314 = call i8* @malloc(i64 %t313)
  call void @__hulk_gc_track(i8* %t314)
  call i8* @strcpy(i8* %t314, i8* %t305)
  call i8* @strcat(i8* %t314, i8* %t309)
  %t315 = ptrtoint i8* %t314 to i64
  %t301 = bitcast i64 %t315 to double
  store double %t301, double* %t285
  br label %merge_288
else_287:
  %t316 = ptrtoint i8* %self to i64
  %t317 = bitcast i64 %t316 to double
  %t318 = bitcast double %t317 to i64
  %t319 = alloca i64
  store i64 %t318, i64* %t319
  %t320 = load i64, i64* %t319
  %t321 = inttoptr i64 %t320 to i8*
  %t322 = bitcast i8* %t321 to %T.Cat*
  %t323 = getelementptr inbounds %T.Cat, %T.Cat* %t322, i32 0, i32 1
  %t324 = load double, double* %t323
  %t326 = ptrtoint i8* getelementptr inbounds ([15 x i8], [15 x i8]* @.slit_325, i64 0, i64 0) to i64
  %t327 = bitcast i64 %t326 to double
  %t329 = bitcast double %t324 to i64
  %t330 = alloca i64
  store i64 %t329, i64* %t330
  %t331 = load i64, i64* %t330
  %t332 = inttoptr i64 %t331 to i8*
  %t333 = bitcast double %t327 to i64
  %t334 = alloca i64
  store i64 %t333, i64* %t334
  %t335 = load i64, i64* %t334
  %t336 = inttoptr i64 %t335 to i8*
  %t337 = call i64 @strlen(i8* %t332)
  %t338 = call i64 @strlen(i8* %t336)
  %t339 = add i64 %t337, %t338
  %t340 = add i64 %t339, 1
  %t341 = call i8* @malloc(i64 %t340)
  call void @__hulk_gc_track(i8* %t341)
  call i8* @strcpy(i8* %t341, i8* %t332)
  call i8* @strcat(i8* %t341, i8* %t336)
  %t342 = ptrtoint i8* %t341 to i64
  %t328 = bitcast i64 %t342 to double
  store double %t328, double* %t285
  br label %merge_288
merge_288:
  %t343 = load double, double* %t285
  ret double %t343
}

define i8* @Doubler_new() {
entry:
  %t344 = call i8* @malloc(i64 8)
  call void @__hulk_gc_track(i8* %t344)
  %t345 = bitcast i8* %t344 to %T.Doubler*
  %t346 = getelementptr inbounds %T.Doubler, %T.Doubler* %t345, i32 0, i32 0
  store i64 6, i64* %t346
  ret i8* %t344
}

define double @Doubler_invoke(i8* %self, double %x) {
entry:
  %t347 = alloca double
  store double %x, double* %t347
  %t348 = load double, double* %t347
  %t349 = fmul double %t348, 2.0e0
  ret double %t349
}

define i8* @Dog_new(double %name, double %breed) {
entry:
  %t350 = call i8* @malloc(i64 40)
  call void @__hulk_gc_track(i8* %t350)
  %t351 = bitcast i8* %t350 to %T.Dog*
  %t352 = getelementptr inbounds %T.Dog, %T.Dog* %t351, i32 0, i32 0
  store i64 7, i64* %t352
  %t353 = alloca double
  store double %name, double* %t353
  %t354 = alloca double
  store double %breed, double* %t354
  %t355 = load double, double* %t353
  %t356 = call i8* @Mammal_new(double %t355, double 4.0e0, double 1.0)
  %t357 = bitcast i8* %t356 to %T.Mammal*
  %t358 = getelementptr inbounds %T.Mammal, %T.Mammal* %t357, i32 0, i32 1
  %t359 = load double, double* %t358
  %t360 = getelementptr inbounds %T.Dog, %T.Dog* %t351, i32 0, i32 1
  store double %t359, double* %t360
  %t361 = getelementptr inbounds %T.Mammal, %T.Mammal* %t357, i32 0, i32 2
  %t362 = load double, double* %t361
  %t363 = getelementptr inbounds %T.Dog, %T.Dog* %t351, i32 0, i32 2
  store double %t362, double* %t363
  %t364 = getelementptr inbounds %T.Mammal, %T.Mammal* %t357, i32 0, i32 3
  %t365 = load double, double* %t364
  %t366 = getelementptr inbounds %T.Dog, %T.Dog* %t351, i32 0, i32 3
  store double %t365, double* %t366
  %t367 = load double, double* %t354
  %t368 = getelementptr inbounds %T.Dog, %T.Dog* %t351, i32 0, i32 4
  store double %t367, double* %t368
  ret i8* %t350
}

define double @Dog_bark(i8* %self) {
entry:
  %t369 = ptrtoint i8* %self to i64
  %t370 = bitcast i64 %t369 to double
  %t371 = bitcast double %t370 to i64
  %t372 = alloca i64
  store i64 %t371, i64* %t372
  %t373 = load i64, i64* %t372
  %t374 = inttoptr i64 %t373 to i8*
  %t375 = bitcast i8* %t374 to %T.Dog*
  %t376 = getelementptr inbounds %T.Dog, %T.Dog* %t375, i32 0, i32 1
  %t377 = load double, double* %t376
  %t379 = ptrtoint i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.slit_378, i64 0, i64 0) to i64
  %t380 = bitcast i64 %t379 to double
  %t382 = bitcast double %t377 to i64
  %t383 = alloca i64
  store i64 %t382, i64* %t383
  %t384 = load i64, i64* %t383
  %t385 = inttoptr i64 %t384 to i8*
  %t386 = bitcast double %t380 to i64
  %t387 = alloca i64
  store i64 %t386, i64* %t387
  %t388 = load i64, i64* %t387
  %t389 = inttoptr i64 %t388 to i8*
  %t390 = call i64 @strlen(i8* %t385)
  %t391 = call i64 @strlen(i8* %t389)
  %t392 = add i64 %t390, %t391
  %t393 = add i64 %t392, 1
  %t394 = call i8* @malloc(i64 %t393)
  call void @__hulk_gc_track(i8* %t394)
  call i8* @strcpy(i8* %t394, i8* %t385)
  call i8* @strcat(i8* %t394, i8* %t389)
  %t395 = ptrtoint i8* %t394 to i64
  %t381 = bitcast i64 %t395 to double
  ret double %t381
}

define double @Dog_describe(i8* %self) {
entry:
  %t396 = ptrtoint i8* %self to i64
  %t397 = bitcast i64 %t396 to double
  %t398 = bitcast double %t397 to i64
  %t399 = alloca i64
  store i64 %t398, i64* %t399
  %t400 = load i64, i64* %t399
  %t401 = inttoptr i64 %t400 to i8*
  %t402 = bitcast i8* %t401 to %T.Dog*
  %t403 = getelementptr inbounds %T.Dog, %T.Dog* %t402, i32 0, i32 1
  %t404 = load double, double* %t403
  %t406 = ptrtoint i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.slit_405, i64 0, i64 0) to i64
  %t407 = bitcast i64 %t406 to double
  %t409 = bitcast double %t404 to i64
  %t410 = alloca i64
  store i64 %t409, i64* %t410
  %t411 = load i64, i64* %t410
  %t412 = inttoptr i64 %t411 to i8*
  %t413 = bitcast double %t407 to i64
  %t414 = alloca i64
  store i64 %t413, i64* %t414
  %t415 = load i64, i64* %t414
  %t416 = inttoptr i64 %t415 to i8*
  %t417 = call i64 @strlen(i8* %t412)
  %t418 = call i64 @strlen(i8* %t416)
  %t419 = add i64 %t417, %t418
  %t420 = add i64 %t419, 1
  %t421 = call i8* @malloc(i64 %t420)
  call void @__hulk_gc_track(i8* %t421)
  call i8* @strcpy(i8* %t421, i8* %t412)
  call i8* @strcat(i8* %t421, i8* %t416)
  %t422 = ptrtoint i8* %t421 to i64
  %t408 = bitcast i64 %t422 to double
  %t423 = ptrtoint i8* %self to i64
  %t424 = bitcast i64 %t423 to double
  %t425 = bitcast double %t424 to i64
  %t426 = alloca i64
  store i64 %t425, i64* %t426
  %t427 = load i64, i64* %t426
  %t428 = inttoptr i64 %t427 to i8*
  %t429 = bitcast i8* %t428 to %T.Dog*
  %t430 = getelementptr inbounds %T.Dog, %T.Dog* %t429, i32 0, i32 4
  %t431 = load double, double* %t430
  %t433 = bitcast double %t408 to i64
  %t434 = alloca i64
  store i64 %t433, i64* %t434
  %t435 = load i64, i64* %t434
  %t436 = inttoptr i64 %t435 to i8*
  %t437 = bitcast double %t431 to i64
  %t438 = alloca i64
  store i64 %t437, i64* %t438
  %t439 = load i64, i64* %t438
  %t440 = inttoptr i64 %t439 to i8*
  %t441 = call i64 @strlen(i8* %t436)
  %t442 = call i64 @strlen(i8* %t440)
  %t443 = add i64 %t441, %t442
  %t444 = add i64 %t443, 1
  %t445 = call i8* @malloc(i64 %t444)
  call void @__hulk_gc_track(i8* %t445)
  call i8* @strcpy(i8* %t445, i8* %t436)
  call i8* @strcat(i8* %t445, i8* %t440)
  %t446 = ptrtoint i8* %t445 to i64
  %t432 = bitcast i64 %t446 to double
  ret double %t432
}

define i8* @Bird_new(double %name, double %can_fly) {
entry:
  %t447 = call i8* @malloc(i64 32)
  call void @__hulk_gc_track(i8* %t447)
  %t448 = bitcast i8* %t447 to %T.Bird*
  %t449 = getelementptr inbounds %T.Bird, %T.Bird* %t448, i32 0, i32 0
  store i64 8, i64* %t449
  %t450 = alloca double
  store double %name, double* %t450
  %t451 = alloca double
  store double %can_fly, double* %t451
  %t452 = load double, double* %t450
  %t453 = call i8* @Animal_new(double %t452, double 2.0e0)
  %t454 = bitcast i8* %t453 to %T.Animal*
  %t455 = getelementptr inbounds %T.Animal, %T.Animal* %t454, i32 0, i32 1
  %t456 = load double, double* %t455
  %t457 = getelementptr inbounds %T.Bird, %T.Bird* %t448, i32 0, i32 1
  store double %t456, double* %t457
  %t458 = getelementptr inbounds %T.Animal, %T.Animal* %t454, i32 0, i32 2
  %t459 = load double, double* %t458
  %t460 = getelementptr inbounds %T.Bird, %T.Bird* %t448, i32 0, i32 2
  store double %t459, double* %t460
  %t461 = load double, double* %t451
  %t462 = getelementptr inbounds %T.Bird, %T.Bird* %t448, i32 0, i32 3
  store double %t461, double* %t462
  ret i8* %t447
}

define double @Bird_describe(i8* %self) {
entry:
  %t463 = ptrtoint i8* %self to i64
  %t464 = bitcast i64 %t463 to double
  %t465 = bitcast double %t464 to i64
  %t466 = alloca i64
  store i64 %t465, i64* %t466
  %t467 = load i64, i64* %t466
  %t468 = inttoptr i64 %t467 to i8*
  %t469 = bitcast i8* %t468 to %T.Bird*
  %t470 = getelementptr inbounds %T.Bird, %T.Bird* %t469, i32 0, i32 3
  %t471 = load double, double* %t470
  %t472 = fcmp one double %t471, 0.0
  %t473 = alloca double
  br i1 %t472, label %then_474, label %else_475
then_474:
  %t477 = ptrtoint i8* %self to i64
  %t478 = bitcast i64 %t477 to double
  %t479 = bitcast double %t478 to i64
  %t480 = alloca i64
  store i64 %t479, i64* %t480
  %t481 = load i64, i64* %t480
  %t482 = inttoptr i64 %t481 to i8*
  %t483 = bitcast i8* %t482 to %T.Bird*
  %t484 = getelementptr inbounds %T.Bird, %T.Bird* %t483, i32 0, i32 1
  %t485 = load double, double* %t484
  %t487 = ptrtoint i8* getelementptr inbounds ([10 x i8], [10 x i8]* @.slit_486, i64 0, i64 0) to i64
  %t488 = bitcast i64 %t487 to double
  %t490 = bitcast double %t485 to i64
  %t491 = alloca i64
  store i64 %t490, i64* %t491
  %t492 = load i64, i64* %t491
  %t493 = inttoptr i64 %t492 to i8*
  %t494 = bitcast double %t488 to i64
  %t495 = alloca i64
  store i64 %t494, i64* %t495
  %t496 = load i64, i64* %t495
  %t497 = inttoptr i64 %t496 to i8*
  %t498 = call i64 @strlen(i8* %t493)
  %t499 = call i64 @strlen(i8* %t497)
  %t500 = add i64 %t498, %t499
  %t501 = add i64 %t500, 1
  %t502 = call i8* @malloc(i64 %t501)
  call void @__hulk_gc_track(i8* %t502)
  call i8* @strcpy(i8* %t502, i8* %t493)
  call i8* @strcat(i8* %t502, i8* %t497)
  %t503 = ptrtoint i8* %t502 to i64
  %t489 = bitcast i64 %t503 to double
  store double %t489, double* %t473
  br label %merge_476
else_475:
  %t504 = ptrtoint i8* %self to i64
  %t505 = bitcast i64 %t504 to double
  %t506 = bitcast double %t505 to i64
  %t507 = alloca i64
  store i64 %t506, i64* %t507
  %t508 = load i64, i64* %t507
  %t509 = inttoptr i64 %t508 to i8*
  %t510 = bitcast i8* %t509 to %T.Bird*
  %t511 = getelementptr inbounds %T.Bird, %T.Bird* %t510, i32 0, i32 1
  %t512 = load double, double* %t511
  %t514 = ptrtoint i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.slit_513, i64 0, i64 0) to i64
  %t515 = bitcast i64 %t514 to double
  %t517 = bitcast double %t512 to i64
  %t518 = alloca i64
  store i64 %t517, i64* %t518
  %t519 = load i64, i64* %t518
  %t520 = inttoptr i64 %t519 to i8*
  %t521 = bitcast double %t515 to i64
  %t522 = alloca i64
  store i64 %t521, i64* %t522
  %t523 = load i64, i64* %t522
  %t524 = inttoptr i64 %t523 to i8*
  %t525 = call i64 @strlen(i8* %t520)
  %t526 = call i64 @strlen(i8* %t524)
  %t527 = add i64 %t525, %t526
  %t528 = add i64 %t527, 1
  %t529 = call i8* @malloc(i64 %t528)
  call void @__hulk_gc_track(i8* %t529)
  call i8* @strcpy(i8* %t529, i8* %t520)
  call i8* @strcat(i8* %t529, i8* %t524)
  %t530 = ptrtoint i8* %t529 to i64
  %t516 = bitcast i64 %t530 to double
  store double %t516, double* %t473
  br label %merge_476
merge_476:
  %t531 = load double, double* %t473
  ret double %t531
}

define double @separator() {
entry:
  %t533 = ptrtoint i8* getelementptr inbounds ([97 x i8], [97 x i8]* @.slit_532, i64 0, i64 0) to i64
  %t534 = bitcast i64 %t533 to double
  %t535 = bitcast double %t534 to i64
  %t536 = alloca i64
  store i64 %t535, i64* %t536
  %t537 = load i64, i64* %t536
  %t538 = inttoptr i64 %t537 to i8*
  call i32 @puts(i8* %t538)
  ret double 0.0
}

define double @abs(double %x) {
entry:
  %t539 = alloca double
  store double %x, double* %t539
  %t540 = load double, double* %t539
  %t542 = fcmp olt double %t540, 0.0e0
  %t541 = uitofp i1 %t542 to double
  %t543 = fcmp one double %t541, 0.0
  %t544 = alloca double
  br i1 %t543, label %then_545, label %else_546
then_545:
  %t548 = load double, double* %t539
  %t549 = fsub double 0.0e0, %t548
  store double %t549, double* %t544
  br label %merge_547
else_546:
  %t550 = load double, double* %t539
  store double %t550, double* %t544
  br label %merge_547
merge_547:
  %t551 = load double, double* %t544
  ret double %t551
}

define double @max(double %a, double %b) {
entry:
  %t552 = alloca double
  store double %a, double* %t552
  %t553 = alloca double
  store double %b, double* %t553
  %t554 = load double, double* %t552
  %t555 = load double, double* %t553
  %t557 = fcmp ogt double %t554, %t555
  %t556 = uitofp i1 %t557 to double
  %t558 = fcmp one double %t556, 0.0
  %t559 = alloca double
  br i1 %t558, label %then_560, label %else_561
then_560:
  %t563 = load double, double* %t552
  store double %t563, double* %t559
  br label %merge_562
else_561:
  %t564 = load double, double* %t553
  store double %t564, double* %t559
  br label %merge_562
merge_562:
  %t565 = load double, double* %t559
  ret double %t565
}

define double @min(double %a, double %b) {
entry:
  %t566 = alloca double
  store double %a, double* %t566
  %t567 = alloca double
  store double %b, double* %t567
  %t568 = load double, double* %t566
  %t569 = load double, double* %t567
  %t571 = fcmp olt double %t568, %t569
  %t570 = uitofp i1 %t571 to double
  %t572 = fcmp one double %t570, 0.0
  %t573 = alloca double
  br i1 %t572, label %then_574, label %else_575
then_574:
  %t577 = load double, double* %t566
  store double %t577, double* %t573
  br label %merge_576
else_575:
  %t578 = load double, double* %t567
  store double %t578, double* %t573
  br label %merge_576
merge_576:
  %t579 = load double, double* %t573
  ret double %t579
}

define double @clamp(double %val, double %lo, double %hi) {
entry:
  %t580 = alloca double
  store double %val, double* %t580
  %t581 = alloca double
  store double %lo, double* %t581
  %t582 = alloca double
  store double %hi, double* %t582
  %t583 = load double, double* %t580
  %t584 = load double, double* %t581
  %t585 = call double @max(double %t583, double %t584)
  %t586 = load double, double* %t582
  %t587 = call double @min(double %t585, double %t586)
  ret double %t587
}

define double @repeat_str(double %s, double %n) {
entry:
  %t588 = alloca double
  store double %s, double* %t588
  %t589 = alloca double
  store double %n, double* %t589
  %t591 = ptrtoint i8* getelementptr inbounds ([1 x i8], [1 x i8]* @.slit_590, i64 0, i64 0) to i64
  %t592 = bitcast i64 %t591 to double
  %t593 = alloca double
  store double %t592, double* %t593
  %t594 = alloca double
  store double 0.0e0, double* %t594
  br label %wcond_595
wcond_595:
  %t598 = load double, double* %t594
  %t599 = load double, double* %t589
  %t601 = fcmp olt double %t598, %t599
  %t600 = uitofp i1 %t601 to double
  %t602 = fcmp one double %t600, 0.0
  br i1 %t602, label %wbody_596, label %wend_597
wbody_596:
  %t603 = load double, double* %t593
  %t604 = load double, double* %t588
  %t606 = bitcast double %t603 to i64
  %t607 = alloca i64
  store i64 %t606, i64* %t607
  %t608 = load i64, i64* %t607
  %t609 = inttoptr i64 %t608 to i8*
  %t610 = bitcast double %t604 to i64
  %t611 = alloca i64
  store i64 %t610, i64* %t611
  %t612 = load i64, i64* %t611
  %t613 = inttoptr i64 %t612 to i8*
  %t614 = call i64 @strlen(i8* %t609)
  %t615 = call i64 @strlen(i8* %t613)
  %t616 = add i64 %t614, %t615
  %t617 = add i64 %t616, 1
  %t618 = call i8* @malloc(i64 %t617)
  call void @__hulk_gc_track(i8* %t618)
  call i8* @strcpy(i8* %t618, i8* %t609)
  call i8* @strcat(i8* %t618, i8* %t613)
  %t619 = ptrtoint i8* %t618 to i64
  %t605 = bitcast i64 %t619 to double
  store double %t605, double* %t593
  %t620 = load double, double* %t594
  %t621 = fadd double %t620, 1.0e0
  store double %t621, double* %t594
  br label %wcond_595
wend_597:
  %t622 = load double, double* %t593
  ret double %t622
}

define void @__hulk_gc_track(i8* %ptr) {
entry:
  %len = load i64, i64* @.gc_len
  %cap = load i64, i64* @.gc_cap
  %need_grow = icmp sge i64 %len, %cap
  br i1 %need_grow, label %grow, label %store

grow:
  %new_cap_base = mul i64 %cap, 2
  %new_cap_min  = add i64 %new_cap_base, 16
  %new_cap = select i1 %need_grow, i64 %new_cap_min, i64 %new_cap_base
  %byte_sz = mul i64 %new_cap, 8
  %old_buf = load i8**, i8*** @.gc_buf
  %old_raw = bitcast i8** %old_buf to i8*
  %new_raw = call i8* @realloc(i8* %old_raw, i64 %byte_sz)
  %new_buf = bitcast i8* %new_raw to i8**
  store i8** %new_buf, i8*** @.gc_buf
  store i64 %new_cap, i64* @.gc_cap
  br label %store

store:
  %cur_buf = load i8**, i8*** @.gc_buf
  %cur_len = load i64, i64* @.gc_len
  %slot = getelementptr i8*, i8** %cur_buf, i64 %cur_len
  store i8* %ptr, i8** %slot
  %new_len = add i64 %cur_len, 1
  store i64 %new_len, i64* @.gc_len
  ret void
}

define void @__hulk_gc_sweep() {
entry:
  %len = load i64, i64* @.gc_len
  %cmp0 = icmp sle i64 %len, 0
  br i1 %cmp0, label %done, label %loop_hdr

loop_hdr:
  %idx = alloca i64
  store i64 0, i64* %idx
  br label %loop

loop:
  %i = load i64, i64* %idx
  %cond = icmp slt i64 %i, %len
  br i1 %cond, label %body, label %free_buf

body:
  %buf = load i8**, i8*** @.gc_buf
  %slot = getelementptr i8*, i8** %buf, i64 %i
  %ptr = load i8*, i8** %slot
  call void @free(i8* %ptr)
  %next = add i64 %i, 1
  store i64 %next, i64* %idx
  br label %loop

free_buf:
  %buf2 = load i8**, i8*** @.gc_buf
  %buf_raw = bitcast i8** %buf2 to i8*
  call void @free(i8* %buf_raw)
  store i8** null, i8*** @.gc_buf
  store i64 0, i64* @.gc_len
  store i64 0, i64* @.gc_cap
  br label %done

done:
  ret void
}

define i8* @__hulk_num_to_str(double %val) {
entry:
  ; First pass: measure needed length
  %len = call i32 (i8*, i64, i8*, ...) @snprintf(i8* null, i64 0, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double %val)
  %len64 = sext i32 %len to i64
  %bufsz = add i64 %len64, 1
  %buf = call i8* @malloc(i64 %bufsz)
  call void @__hulk_gc_track(i8* %buf)
  ; Second pass: actually format
  call i32 (i8*, i64, i8*, ...) @snprintf(i8* %buf, i64 %bufsz, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double %val)
  ret i8* %buf
}

define i8* @__hulk_bool_to_str(double %val) {
entry:
  %cond = fcmp one double %val, 0.0
  %res = select i1 %cond, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  ret i8* %res
}

define void @__hulk_print_val(double %val) {
entry:
  ; Fallback: print as number (safe default for Unknown type hint)
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double %val)
  call i32 @puts(i8* getelementptr inbounds ([1 x i8], [1 x i8]* @.empty_s, i64 0, i64 0))
  ret void
}

define i8* @__hulk_to_str(double %val) {
entry:
  %numstr = call i8* @__hulk_num_to_str(double %val)
  ret i8* %numstr
}

define i32 @main() {
entry:
  %t624 = ptrtoint i8* getelementptr inbounds ([30 x i8], [30 x i8]* @.slit_623, i64 0, i64 0) to i64
  %t625 = bitcast i64 %t624 to double
  %t626 = bitcast double %t625 to i64
  %t627 = alloca i64
  store i64 %t626, i64* %t627
  %t628 = load i64, i64* %t627
  %t629 = inttoptr i64 %t628 to i8*
  call i32 @puts(i8* %t629)
  %t630 = call double @separator()
  %t632 = ptrtoint i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.slit_631, i64 0, i64 0) to i64
  %t633 = bitcast i64 %t632 to double
  %t635 = ptrtoint i8* getelementptr inbounds ([9 x i8], [9 x i8]* @.slit_634, i64 0, i64 0) to i64
  %t636 = bitcast i64 %t635 to double
  %t637 = call i8* @Dog_new(double %t633, double %t636)
  %t638 = ptrtoint i8* %t637 to i64
  %t639 = bitcast i64 %t638 to double
  %t640 = alloca double
  store double %t639, double* %t640
  %t642 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_641, i64 0, i64 0) to i64
  %t643 = bitcast i64 %t642 to double
  %t645 = ptrtoint i8* getelementptr inbounds ([7 x i8], [7 x i8]* @.slit_644, i64 0, i64 0) to i64
  %t646 = bitcast i64 %t645 to double
  %t647 = call i8* @Dog_new(double %t643, double %t646)
  %t648 = ptrtoint i8* %t647 to i64
  %t649 = bitcast i64 %t648 to double
  %t650 = alloca double
  store double %t649, double* %t650
  %t652 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_651, i64 0, i64 0) to i64
  %t653 = bitcast i64 %t652 to double
  %t654 = call i8* @Cat_new(double %t653, double 1.0)
  %t655 = ptrtoint i8* %t654 to i64
  %t656 = bitcast i64 %t655 to double
  %t657 = alloca double
  store double %t656, double* %t657
  %t659 = ptrtoint i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.slit_658, i64 0, i64 0) to i64
  %t660 = bitcast i64 %t659 to double
  %t661 = call i8* @Cat_new(double %t660, double 0.0)
  %t662 = ptrtoint i8* %t661 to i64
  %t663 = bitcast i64 %t662 to double
  %t664 = alloca double
  store double %t663, double* %t664
  %t666 = ptrtoint i8* getelementptr inbounds ([7 x i8], [7 x i8]* @.slit_665, i64 0, i64 0) to i64
  %t667 = bitcast i64 %t666 to double
  %t668 = call i8* @Bird_new(double %t667, double 1.0)
  %t669 = ptrtoint i8* %t668 to i64
  %t670 = bitcast i64 %t669 to double
  %t671 = alloca double
  store double %t670, double* %t671
  %t673 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_672, i64 0, i64 0) to i64
  %t674 = bitcast i64 %t673 to double
  %t675 = call i8* @Bird_new(double %t674, double 0.0)
  %t676 = ptrtoint i8* %t675 to i64
  %t677 = bitcast i64 %t676 to double
  %t678 = alloca double
  store double %t677, double* %t678
  %t680 = ptrtoint i8* getelementptr inbounds ([25 x i8], [25 x i8]* @.slit_679, i64 0, i64 0) to i64
  %t681 = bitcast i64 %t680 to double
  %t682 = bitcast double %t681 to i64
  %t683 = alloca i64
  store i64 %t682, i64* %t683
  %t684 = load i64, i64* %t683
  %t685 = inttoptr i64 %t684 to i8*
  call i32 @puts(i8* %t685)
  %t686 = load double, double* %t640
  %t687 = bitcast double %t686 to i64
  %t688 = alloca i64
  store i64 %t687, i64* %t688
  %t689 = load i64, i64* %t688
  %t690 = inttoptr i64 %t689 to i8*
  %t691 = call double @Animal_greet(i8* %t690)
  %t692 = bitcast double %t691 to i64
  %t693 = alloca i64
  store i64 %t692, i64* %t693
  %t694 = load i64, i64* %t693
  %t695 = inttoptr i64 %t694 to i8*
  call i32 @puts(i8* %t695)
  %t696 = load double, double* %t657
  %t697 = bitcast double %t696 to i64
  %t698 = alloca i64
  store i64 %t697, i64* %t698
  %t699 = load i64, i64* %t698
  %t700 = inttoptr i64 %t699 to i8*
  %t701 = call double @Animal_greet(i8* %t700)
  %t702 = bitcast double %t701 to i64
  %t703 = alloca i64
  store i64 %t702, i64* %t703
  %t704 = load i64, i64* %t703
  %t705 = inttoptr i64 %t704 to i8*
  call i32 @puts(i8* %t705)
  %t706 = load double, double* %t671
  %t707 = bitcast double %t706 to i64
  %t708 = alloca i64
  store i64 %t707, i64* %t708
  %t709 = load i64, i64* %t708
  %t710 = inttoptr i64 %t709 to i8*
  %t711 = call double @Animal_greet(i8* %t710)
  %t712 = bitcast double %t711 to i64
  %t713 = alloca i64
  store i64 %t712, i64* %t713
  %t714 = load i64, i64* %t713
  %t715 = inttoptr i64 %t714 to i8*
  call i32 @puts(i8* %t715)
  %t716 = call double @separator()
  %t718 = ptrtoint i8* getelementptr inbounds ([20 x i8], [20 x i8]* @.slit_717, i64 0, i64 0) to i64
  %t719 = bitcast i64 %t718 to double
  %t720 = bitcast double %t719 to i64
  %t721 = alloca i64
  store i64 %t720, i64* %t721
  %t722 = load i64, i64* %t721
  %t723 = inttoptr i64 %t722 to i8*
  call i32 @puts(i8* %t723)
  %t724 = load double, double* %t640
  %t725 = bitcast double %t724 to i64
  %t726 = alloca i64
  store i64 %t725, i64* %t726
  %t727 = load i64, i64* %t726
  %t728 = inttoptr i64 %t727 to i8*
  %t729 = call double @Dog_bark(i8* %t728)
  %t730 = bitcast double %t729 to i64
  %t731 = alloca i64
  store i64 %t730, i64* %t731
  %t732 = load i64, i64* %t731
  %t733 = inttoptr i64 %t732 to i8*
  call i32 @puts(i8* %t733)
  %t734 = load double, double* %t650
  %t735 = bitcast double %t734 to i64
  %t736 = alloca i64
  store i64 %t735, i64* %t736
  %t737 = load i64, i64* %t736
  %t738 = inttoptr i64 %t737 to i8*
  %t739 = call double @Dog_bark(i8* %t738)
  %t740 = bitcast double %t739 to i64
  %t741 = alloca i64
  store i64 %t740, i64* %t741
  %t742 = load i64, i64* %t741
  %t743 = inttoptr i64 %t742 to i8*
  call i32 @puts(i8* %t743)
  %t744 = load double, double* %t640
  %t745 = bitcast double %t744 to i64
  %t746 = alloca i64
  store i64 %t745, i64* %t746
  %t747 = load i64, i64* %t746
  %t748 = inttoptr i64 %t747 to i8*
  %t749 = call double @Dog_describe(i8* %t748)
  %t750 = bitcast double %t749 to i64
  %t751 = alloca i64
  store i64 %t750, i64* %t751
  %t752 = load i64, i64* %t751
  %t753 = inttoptr i64 %t752 to i8*
  call i32 @puts(i8* %t753)
  %t754 = load double, double* %t650
  %t755 = bitcast double %t754 to i64
  %t756 = alloca i64
  store i64 %t755, i64* %t756
  %t757 = load i64, i64* %t756
  %t758 = inttoptr i64 %t757 to i8*
  %t759 = call double @Dog_describe(i8* %t758)
  %t760 = bitcast double %t759 to i64
  %t761 = alloca i64
  store i64 %t760, i64* %t761
  %t762 = load i64, i64* %t761
  %t763 = inttoptr i64 %t762 to i8*
  call i32 @puts(i8* %t763)
  %t764 = call double @separator()
  %t766 = ptrtoint i8* getelementptr inbounds ([20 x i8], [20 x i8]* @.slit_765, i64 0, i64 0) to i64
  %t767 = bitcast i64 %t766 to double
  %t768 = bitcast double %t767 to i64
  %t769 = alloca i64
  store i64 %t768, i64* %t769
  %t770 = load i64, i64* %t769
  %t771 = inttoptr i64 %t770 to i8*
  call i32 @puts(i8* %t771)
  %t772 = load double, double* %t657
  %t773 = bitcast double %t772 to i64
  %t774 = alloca i64
  store i64 %t773, i64* %t774
  %t775 = load i64, i64* %t774
  %t776 = inttoptr i64 %t775 to i8*
  %t777 = call double @Cat_meow(i8* %t776)
  %t778 = bitcast double %t777 to i64
  %t779 = alloca i64
  store i64 %t778, i64* %t779
  %t780 = load i64, i64* %t779
  %t781 = inttoptr i64 %t780 to i8*
  call i32 @puts(i8* %t781)
  %t782 = load double, double* %t664
  %t783 = bitcast double %t782 to i64
  %t784 = alloca i64
  store i64 %t783, i64* %t784
  %t785 = load i64, i64* %t784
  %t786 = inttoptr i64 %t785 to i8*
  %t787 = call double @Cat_meow(i8* %t786)
  %t788 = bitcast double %t787 to i64
  %t789 = alloca i64
  store i64 %t788, i64* %t789
  %t790 = load i64, i64* %t789
  %t791 = inttoptr i64 %t790 to i8*
  call i32 @puts(i8* %t791)
  %t792 = load double, double* %t657
  %t793 = bitcast double %t792 to i64
  %t794 = alloca i64
  store i64 %t793, i64* %t794
  %t795 = load i64, i64* %t794
  %t796 = inttoptr i64 %t795 to i8*
  %t797 = call double @Cat_describe(i8* %t796)
  %t798 = bitcast double %t797 to i64
  %t799 = alloca i64
  store i64 %t798, i64* %t799
  %t800 = load i64, i64* %t799
  %t801 = inttoptr i64 %t800 to i8*
  call i32 @puts(i8* %t801)
  %t802 = load double, double* %t664
  %t803 = bitcast double %t802 to i64
  %t804 = alloca i64
  store i64 %t803, i64* %t804
  %t805 = load i64, i64* %t804
  %t806 = inttoptr i64 %t805 to i8*
  %t807 = call double @Cat_describe(i8* %t806)
  %t808 = bitcast double %t807 to i64
  %t809 = alloca i64
  store i64 %t808, i64* %t809
  %t810 = load i64, i64* %t809
  %t811 = inttoptr i64 %t810 to i8*
  call i32 @puts(i8* %t811)
  %t812 = call double @separator()
  %t814 = ptrtoint i8* getelementptr inbounds ([18 x i8], [18 x i8]* @.slit_813, i64 0, i64 0) to i64
  %t815 = bitcast i64 %t814 to double
  %t816 = bitcast double %t815 to i64
  %t817 = alloca i64
  store i64 %t816, i64* %t817
  %t818 = load i64, i64* %t817
  %t819 = inttoptr i64 %t818 to i8*
  call i32 @puts(i8* %t819)
  %t820 = load double, double* %t671
  %t821 = bitcast double %t820 to i64
  %t822 = alloca i64
  store i64 %t821, i64* %t822
  %t823 = load i64, i64* %t822
  %t824 = inttoptr i64 %t823 to i8*
  %t825 = call double @Bird_describe(i8* %t824)
  %t826 = bitcast double %t825 to i64
  %t827 = alloca i64
  store i64 %t826, i64* %t827
  %t828 = load i64, i64* %t827
  %t829 = inttoptr i64 %t828 to i8*
  call i32 @puts(i8* %t829)
  %t830 = load double, double* %t678
  %t831 = bitcast double %t830 to i64
  %t832 = alloca i64
  store i64 %t831, i64* %t832
  %t833 = load i64, i64* %t832
  %t834 = inttoptr i64 %t833 to i8*
  %t835 = call double @Bird_describe(i8* %t834)
  %t836 = bitcast double %t835 to i64
  %t837 = alloca i64
  store i64 %t836, i64* %t837
  %t838 = load i64, i64* %t837
  %t839 = inttoptr i64 %t838 to i8*
  call i32 @puts(i8* %t839)
  %t840 = call double @separator()
  %t842 = ptrtoint i8* getelementptr inbounds ([26 x i8], [26 x i8]* @.slit_841, i64 0, i64 0) to i64
  %t843 = bitcast i64 %t842 to double
  %t844 = bitcast double %t843 to i64
  %t845 = alloca i64
  store i64 %t844, i64* %t845
  %t846 = load i64, i64* %t845
  %t847 = inttoptr i64 %t846 to i8*
  call i32 @puts(i8* %t847)
  %t848 = load double, double* %t640
  %t849 = bitcast double %t848 to i64
  %t850 = alloca i64
  store i64 %t849, i64* %t850
  %t851 = load i64, i64* %t850
  %t852 = inttoptr i64 %t851 to i8*
  %t853 = call double @Mammal_body_temp(i8* %t852)
  %t854 = bitcast double %t853 to i64
  %t855 = alloca i64
  store i64 %t854, i64* %t855
  %t856 = load i64, i64* %t855
  %t857 = inttoptr i64 %t856 to i8*
  call i32 @puts(i8* %t857)
  %t858 = load double, double* %t657
  %t859 = bitcast double %t858 to i64
  %t860 = alloca i64
  store i64 %t859, i64* %t860
  %t861 = load i64, i64* %t860
  %t862 = inttoptr i64 %t861 to i8*
  %t863 = call double @Mammal_body_temp(i8* %t862)
  %t864 = bitcast double %t863 to i64
  %t865 = alloca i64
  store i64 %t864, i64* %t865
  %t866 = load i64, i64* %t865
  %t867 = inttoptr i64 %t866 to i8*
  call i32 @puts(i8* %t867)
  %t868 = call double @separator()
  %t870 = ptrtoint i8* getelementptr inbounds ([25 x i8], [25 x i8]* @.slit_869, i64 0, i64 0) to i64
  %t871 = bitcast i64 %t870 to double
  %t872 = bitcast double %t871 to i64
  %t873 = alloca i64
  store i64 %t872, i64* %t873
  %t874 = load i64, i64* %t873
  %t875 = inttoptr i64 %t874 to i8*
  call i32 @puts(i8* %t875)
  %t877 = ptrtoint i8* getelementptr inbounds ([15 x i8], [15 x i8]* @.slit_876, i64 0, i64 0) to i64
  %t878 = bitcast i64 %t877 to double
  %t879 = bitcast double %t878 to i64
  %t880 = alloca i64
  store i64 %t879, i64* %t880
  %t881 = load i64, i64* %t880
  %t882 = inttoptr i64 %t881 to i8*
  call i32 @puts(i8* %t882)
  %t883 = load double, double* %t640
  %t884 = bitcast double %t883 to i64
  %t885 = alloca i64
  store i64 %t884, i64* %t885
  %t886 = load i64, i64* %t885
  %t887 = inttoptr i64 %t886 to i64*
  %t888 = load i64, i64* %t887
  %t889 = alloca double
  store double 0.0, double* %t889
  %t890 = icmp eq i64 %t888, 3
  br i1 %t890, label %is_match_891, label %is_next_892
is_match_891:
  store double 1.0, double* %t889
  br label %is_next_892
is_next_892:
  %t893 = icmp eq i64 %t888, 5
  br i1 %t893, label %is_match_894, label %is_next_895
is_match_894:
  store double 1.0, double* %t889
  br label %is_next_895
is_next_895:
  %t896 = icmp eq i64 %t888, 8
  br i1 %t896, label %is_match_897, label %is_next_898
is_match_897:
  store double 1.0, double* %t889
  br label %is_next_898
is_next_898:
  %t899 = icmp eq i64 %t888, 4
  br i1 %t899, label %is_match_900, label %is_next_901
is_match_900:
  store double 1.0, double* %t889
  br label %is_next_901
is_next_901:
  %t902 = icmp eq i64 %t888, 7
  br i1 %t902, label %is_match_903, label %is_next_904
is_match_903:
  store double 1.0, double* %t889
  br label %is_next_904
is_next_904:
  %t905 = load double, double* %t889
  %t906 = fcmp one double %t905, 0.0
  %t907 = select i1 %t906, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t907)
  %t909 = ptrtoint i8* getelementptr inbounds ([15 x i8], [15 x i8]* @.slit_908, i64 0, i64 0) to i64
  %t910 = bitcast i64 %t909 to double
  %t911 = bitcast double %t910 to i64
  %t912 = alloca i64
  store i64 %t911, i64* %t912
  %t913 = load i64, i64* %t912
  %t914 = inttoptr i64 %t913 to i8*
  call i32 @puts(i8* %t914)
  %t915 = load double, double* %t640
  %t916 = bitcast double %t915 to i64
  %t917 = alloca i64
  store i64 %t916, i64* %t917
  %t918 = load i64, i64* %t917
  %t919 = inttoptr i64 %t918 to i64*
  %t920 = load i64, i64* %t919
  %t921 = alloca double
  store double 0.0, double* %t921
  %t922 = icmp eq i64 %t920, 4
  br i1 %t922, label %is_match_923, label %is_next_924
is_match_923:
  store double 1.0, double* %t921
  br label %is_next_924
is_next_924:
  %t925 = icmp eq i64 %t920, 5
  br i1 %t925, label %is_match_926, label %is_next_927
is_match_926:
  store double 1.0, double* %t921
  br label %is_next_927
is_next_927:
  %t928 = icmp eq i64 %t920, 7
  br i1 %t928, label %is_match_929, label %is_next_930
is_match_929:
  store double 1.0, double* %t921
  br label %is_next_930
is_next_930:
  %t931 = load double, double* %t921
  %t932 = fcmp one double %t931, 0.0
  %t933 = select i1 %t932, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t933)
  %t935 = ptrtoint i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.slit_934, i64 0, i64 0) to i64
  %t936 = bitcast i64 %t935 to double
  %t937 = bitcast double %t936 to i64
  %t938 = alloca i64
  store i64 %t937, i64* %t938
  %t939 = load i64, i64* %t938
  %t940 = inttoptr i64 %t939 to i8*
  call i32 @puts(i8* %t940)
  %t941 = load double, double* %t640
  %t942 = bitcast double %t941 to i64
  %t943 = alloca i64
  store i64 %t942, i64* %t943
  %t944 = load i64, i64* %t943
  %t945 = inttoptr i64 %t944 to i64*
  %t946 = load i64, i64* %t945
  %t947 = alloca double
  store double 0.0, double* %t947
  %t948 = icmp eq i64 %t946, 7
  br i1 %t948, label %is_match_949, label %is_next_950
is_match_949:
  store double 1.0, double* %t947
  br label %is_next_950
is_next_950:
  %t951 = load double, double* %t947
  %t952 = fcmp one double %t951, 0.0
  %t953 = select i1 %t952, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t953)
  %t955 = ptrtoint i8* getelementptr inbounds ([18 x i8], [18 x i8]* @.slit_954, i64 0, i64 0) to i64
  %t956 = bitcast i64 %t955 to double
  %t957 = bitcast double %t956 to i64
  %t958 = alloca i64
  store i64 %t957, i64* %t958
  %t959 = load i64, i64* %t958
  %t960 = inttoptr i64 %t959 to i8*
  call i32 @puts(i8* %t960)
  %t961 = load double, double* %t671
  %t962 = bitcast double %t961 to i64
  %t963 = alloca i64
  store i64 %t962, i64* %t963
  %t964 = load i64, i64* %t963
  %t965 = inttoptr i64 %t964 to i64*
  %t966 = load i64, i64* %t965
  %t967 = alloca double
  store double 0.0, double* %t967
  %t968 = icmp eq i64 %t966, 4
  br i1 %t968, label %is_match_969, label %is_next_970
is_match_969:
  store double 1.0, double* %t967
  br label %is_next_970
is_next_970:
  %t971 = icmp eq i64 %t966, 5
  br i1 %t971, label %is_match_972, label %is_next_973
is_match_972:
  store double 1.0, double* %t967
  br label %is_next_973
is_next_973:
  %t974 = icmp eq i64 %t966, 7
  br i1 %t974, label %is_match_975, label %is_next_976
is_match_975:
  store double 1.0, double* %t967
  br label %is_next_976
is_next_976:
  %t977 = load double, double* %t967
  %t978 = fcmp one double %t977, 0.0
  %t979 = select i1 %t978, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t979)
  %t981 = ptrtoint i8* getelementptr inbounds ([18 x i8], [18 x i8]* @.slit_980, i64 0, i64 0) to i64
  %t982 = bitcast i64 %t981 to double
  %t983 = bitcast double %t982 to i64
  %t984 = alloca i64
  store i64 %t983, i64* %t984
  %t985 = load i64, i64* %t984
  %t986 = inttoptr i64 %t985 to i8*
  call i32 @puts(i8* %t986)
  %t987 = load double, double* %t671
  %t988 = bitcast double %t987 to i64
  %t989 = alloca i64
  store i64 %t988, i64* %t989
  %t990 = load i64, i64* %t989
  %t991 = inttoptr i64 %t990 to i64*
  %t992 = load i64, i64* %t991
  %t993 = alloca double
  store double 0.0, double* %t993
  %t994 = icmp eq i64 %t992, 3
  br i1 %t994, label %is_match_995, label %is_next_996
is_match_995:
  store double 1.0, double* %t993
  br label %is_next_996
is_next_996:
  %t997 = icmp eq i64 %t992, 5
  br i1 %t997, label %is_match_998, label %is_next_999
is_match_998:
  store double 1.0, double* %t993
  br label %is_next_999
is_next_999:
  %t1000 = icmp eq i64 %t992, 8
  br i1 %t1000, label %is_match_1001, label %is_next_1002
is_match_1001:
  store double 1.0, double* %t993
  br label %is_next_1002
is_next_1002:
  %t1003 = icmp eq i64 %t992, 4
  br i1 %t1003, label %is_match_1004, label %is_next_1005
is_match_1004:
  store double 1.0, double* %t993
  br label %is_next_1005
is_next_1005:
  %t1006 = icmp eq i64 %t992, 7
  br i1 %t1006, label %is_match_1007, label %is_next_1008
is_match_1007:
  store double 1.0, double* %t993
  br label %is_next_1008
is_next_1008:
  %t1009 = load double, double* %t993
  %t1010 = fcmp one double %t1009, 0.0
  %t1011 = select i1 %t1010, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t1011)
  %t1012 = call double @separator()
  %t1014 = ptrtoint i8* getelementptr inbounds ([29 x i8], [29 x i8]* @.slit_1013, i64 0, i64 0) to i64
  %t1015 = bitcast i64 %t1014 to double
  %t1016 = bitcast double %t1015 to i64
  %t1017 = alloca i64
  store i64 %t1016, i64* %t1017
  %t1018 = load i64, i64* %t1017
  %t1019 = inttoptr i64 %t1018 to i8*
  call i32 @puts(i8* %t1019)
  %t1020 = call i8* @malloc(i64 56)
  call void @__hulk_gc_track(i8* %t1020)
  %t1021 = bitcast i8* %t1020 to double*
  store double 6.0e0, double* %t1021
  %t1022 = getelementptr double, double* %t1021, i64 1
  store double 4.0e0, double* %t1022
  %t1023 = getelementptr double, double* %t1021, i64 2
  store double 4.0e0, double* %t1023
  %t1024 = getelementptr double, double* %t1021, i64 3
  store double 4.0e0, double* %t1024
  %t1025 = getelementptr double, double* %t1021, i64 4
  store double 4.0e0, double* %t1025
  %t1026 = getelementptr double, double* %t1021, i64 5
  store double 2.0e0, double* %t1026
  %t1027 = getelementptr double, double* %t1021, i64 6
  store double 2.0e0, double* %t1027
  %t1028 = ptrtoint double* %t1021 to i64
  %t1029 = bitcast i64 %t1028 to double
  %t1030 = alloca double
  store double %t1029, double* %t1030
  %t1031 = alloca double
  store double 0.0e0, double* %t1031
  %t1032 = load double, double* %t1030
  %t1033 = bitcast double %t1032 to i64
  %t1034 = alloca i64
  store i64 %t1033, i64* %t1034
  %t1035 = load i64, i64* %t1034
  %t1036 = inttoptr i64 %t1035 to double*
  %t1037 = load double, double* %t1036
  %t1038 = fptosi double %t1037 to i64
  %t1039 = alloca i64
  store i64 0, i64* %t1039
  br label %fcond_1040
fcond_1040:
  %t1043 = load i64, i64* %t1039
  %t1044 = icmp slt i64 %t1043, %t1038
  br i1 %t1044, label %fbody_1041, label %fend_1042
fbody_1041:
  %t1045 = load i64, i64* %t1039
  %t1046 = add i64 %t1045, 1
  %t1047 = getelementptr double, double* %t1036, i64 %t1046
  %t1048 = load double, double* %t1047
  %t1049 = alloca double
  store double %t1048, double* %t1049
  %t1050 = load double, double* %t1031
  %t1051 = load double, double* %t1049
  %t1052 = fadd double %t1050, %t1051
  store double %t1052, double* %t1031
  %t1053 = load i64, i64* %t1039
  %t1054 = add i64 %t1053, 1
  store i64 %t1054, i64* %t1039
  br label %fcond_1040
fend_1042:
  %t1056 = ptrtoint i8* getelementptr inbounds ([19 x i8], [19 x i8]* @.slit_1055, i64 0, i64 0) to i64
  %t1057 = bitcast i64 %t1056 to double
  %t1058 = load double, double* %t1031
  %t1060 = bitcast double %t1057 to i64
  %t1061 = alloca i64
  store i64 %t1060, i64* %t1061
  %t1062 = load i64, i64* %t1061
  %t1063 = inttoptr i64 %t1062 to i8*
  %t1064 = call i8* @__hulk_num_to_str(double %t1058)
  %t1065 = call i64 @strlen(i8* %t1063)
  %t1066 = call i64 @strlen(i8* %t1064)
  %t1067 = add i64 %t1065, %t1066
  %t1068 = add i64 %t1067, 2
  %t1069 = call i8* @malloc(i64 %t1068)
  call void @__hulk_gc_track(i8* %t1069)
  call i8* @strcpy(i8* %t1069, i8* %t1063)
  call i8* @strcat(i8* %t1069, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1069, i8* %t1064)
  %t1070 = ptrtoint i8* %t1069 to i64
  %t1059 = bitcast i64 %t1070 to double
  %t1071 = bitcast double %t1059 to i64
  %t1072 = alloca i64
  store i64 %t1071, i64* %t1072
  %t1073 = load i64, i64* %t1072
  %t1074 = inttoptr i64 %t1073 to i8*
  call i32 @puts(i8* %t1074)
  %t1075 = load double, double* %t1030
  %t1076 = bitcast double %t1075 to i64
  %t1077 = alloca i64
  store i64 %t1076, i64* %t1077
  %t1078 = load i64, i64* %t1077
  %t1079 = inttoptr i64 %t1078 to double*
  %t1080 = load double, double* %t1079
  %t1081 = fptosi double %t1080 to i64
  %t1082 = add i64 %t1081, 1
  %t1083 = mul i64 %t1082, 8
  %t1084 = call i8* @malloc(i64 %t1083)
  call void @__hulk_gc_track(i8* %t1084)
  %t1085 = bitcast i8* %t1084 to double*
  store double %t1080, double* %t1085
  %t1086 = alloca i64
  store i64 0, i64* %t1086
  br label %vgc_1087
vgc_1087:
  %t1090 = load i64, i64* %t1086
  %t1091 = icmp slt i64 %t1090, %t1081
  br i1 %t1091, label %vgb_1088, label %vge_1089
vgb_1088:
  %t1092 = add i64 %t1090, 1
  %t1093 = getelementptr double, double* %t1079, i64 %t1092
  %t1094 = load double, double* %t1093
  %t1095 = alloca double
  store double %t1094, double* %t1095
  %t1096 = load double, double* %t1095
  %t1097 = fmul double %t1096, 2.0e0
  %t1098 = getelementptr double, double* %t1085, i64 %t1092
  store double %t1097, double* %t1098
  %t1099 = add i64 %t1090, 1
  store i64 %t1099, i64* %t1086
  br label %vgc_1087
vge_1089:
  %t1100 = ptrtoint double* %t1085 to i64
  %t1101 = bitcast i64 %t1100 to double
  %t1102 = alloca double
  store double %t1101, double* %t1102
  %t1104 = ptrtoint i8* getelementptr inbounds ([20 x i8], [20 x i8]* @.slit_1103, i64 0, i64 0) to i64
  %t1105 = bitcast i64 %t1104 to double
  %t1106 = bitcast double %t1105 to i64
  %t1107 = alloca i64
  store i64 %t1106, i64* %t1107
  %t1108 = load i64, i64* %t1107
  %t1109 = inttoptr i64 %t1108 to i8*
  call i32 @puts(i8* %t1109)
  %t1110 = load double, double* %t1102
  %t1111 = bitcast double %t1110 to i64
  %t1112 = alloca i64
  store i64 %t1111, i64* %t1112
  %t1113 = load i64, i64* %t1112
  %t1114 = inttoptr i64 %t1113 to double*
  %t1115 = load double, double* %t1114
  %t1116 = fptosi double %t1115 to i64
  %t1117 = alloca i64
  store i64 0, i64* %t1117
  br label %fcond_1118
fcond_1118:
  %t1121 = load i64, i64* %t1117
  %t1122 = icmp slt i64 %t1121, %t1116
  br i1 %t1122, label %fbody_1119, label %fend_1120
fbody_1119:
  %t1123 = load i64, i64* %t1117
  %t1124 = add i64 %t1123, 1
  %t1125 = getelementptr double, double* %t1114, i64 %t1124
  %t1126 = load double, double* %t1125
  %t1127 = alloca double
  store double %t1126, double* %t1127
  %t1128 = load double, double* %t1127
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double %t1128)
  call i32 @puts(i8* getelementptr inbounds ([1 x i8], [1 x i8]* @.empty_s, i64 0, i64 0))
  %t1129 = load i64, i64* %t1117
  %t1130 = add i64 %t1129, 1
  store i64 %t1130, i64* %t1117
  br label %fcond_1118
fend_1120:
  %t1131 = call double @separator()
  %t1133 = ptrtoint i8* getelementptr inbounds ([31 x i8], [31 x i8]* @.slit_1132, i64 0, i64 0) to i64
  %t1134 = bitcast i64 %t1133 to double
  %t1135 = bitcast double %t1134 to i64
  %t1136 = alloca i64
  store i64 %t1135, i64* %t1136
  %t1137 = load i64, i64* %t1136
  %t1138 = inttoptr i64 %t1137 to i8*
  call i32 @puts(i8* %t1138)
  %t1144 = call i8* @malloc(i64 16)
  call void @__hulk_gc_track(i8* %t1144)
  %t1145 = bitcast i8* %t1144 to double*
  %t1146 = ptrtoint double (double*, double)* @__lambda_1139 to i64
  %t1147 = bitcast i64 %t1146 to double
  store double %t1147, double* %t1145
  %t1148 = getelementptr double, double* %t1145, i64 1
  store double 0.0, double* %t1148
  %t1149 = ptrtoint double* %t1145 to i64
  %t1150 = bitcast i64 %t1149 to double
  %t1151 = alloca double
  store double %t1150, double* %t1151
  %t1153 = ptrtoint i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.slit_1152, i64 0, i64 0) to i64
  %t1154 = bitcast i64 %t1153 to double
  %t1155 = load double, double* %t1151
  %t1156 = bitcast double %t1155 to i64
  %t1157 = alloca i64
  store i64 %t1156, i64* %t1157
  %t1158 = load i64, i64* %t1157
  %t1159 = inttoptr i64 %t1158 to double*
  %t1160 = load double, double* %t1159
  %t1161 = bitcast double %t1160 to i64
  %t1162 = alloca i64
  store i64 %t1161, i64* %t1162
  %t1163 = load i64, i64* %t1162
  %t1164 = inttoptr i64 %t1163 to double (double*, double)*
  %t1165 = getelementptr double, double* %t1159, i64 1
  %t1166 = load double, double* %t1165
  %t1167 = bitcast double %t1166 to i64
  %t1168 = alloca i64
  store i64 %t1167, i64* %t1168
  %t1169 = load i64, i64* %t1168
  %t1170 = inttoptr i64 %t1169 to double*
  %t1171 = call double %t1164(double* %t1170, double 5.0e0)
  %t1173 = bitcast double %t1154 to i64
  %t1174 = alloca i64
  store i64 %t1173, i64* %t1174
  %t1175 = load i64, i64* %t1174
  %t1176 = inttoptr i64 %t1175 to i8*
  %t1177 = call i8* @__hulk_num_to_str(double %t1171)
  %t1178 = call i64 @strlen(i8* %t1176)
  %t1179 = call i64 @strlen(i8* %t1177)
  %t1180 = add i64 %t1178, %t1179
  %t1181 = add i64 %t1180, 2
  %t1182 = call i8* @malloc(i64 %t1181)
  call void @__hulk_gc_track(i8* %t1182)
  call i8* @strcpy(i8* %t1182, i8* %t1176)
  call i8* @strcat(i8* %t1182, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1182, i8* %t1177)
  %t1183 = ptrtoint i8* %t1182 to i64
  %t1172 = bitcast i64 %t1183 to double
  %t1184 = bitcast double %t1172 to i64
  %t1185 = alloca i64
  store i64 %t1184, i64* %t1185
  %t1186 = load i64, i64* %t1185
  %t1187 = inttoptr i64 %t1186 to i8*
  call i32 @puts(i8* %t1187)
  %t1188 = alloca double
  store double 3.0e0, double* %t1188
  %t1193 = call i8* @malloc(i64 16)
  call void @__hulk_gc_track(i8* %t1193)
  %t1194 = bitcast i8* %t1193 to double*
  %t1195 = ptrtoint double (double*, double)* @__lambda_1189 to i64
  %t1196 = bitcast i64 %t1195 to double
  store double %t1196, double* %t1194
  %t1197 = getelementptr double, double* %t1194, i64 1
  store double 0.0, double* %t1197
  %t1198 = ptrtoint double* %t1194 to i64
  %t1199 = bitcast i64 %t1198 to double
  %t1200 = alloca double
  store double %t1199, double* %t1200
  %t1202 = ptrtoint i8* getelementptr inbounds ([17 x i8], [17 x i8]* @.slit_1201, i64 0, i64 0) to i64
  %t1203 = bitcast i64 %t1202 to double
  %t1204 = load double, double* %t1200
  %t1205 = bitcast double %t1204 to i64
  %t1206 = alloca i64
  store i64 %t1205, i64* %t1206
  %t1207 = load i64, i64* %t1206
  %t1208 = inttoptr i64 %t1207 to double*
  %t1209 = load double, double* %t1208
  %t1210 = bitcast double %t1209 to i64
  %t1211 = alloca i64
  store i64 %t1210, i64* %t1211
  %t1212 = load i64, i64* %t1211
  %t1213 = inttoptr i64 %t1212 to double (double*, double)*
  %t1214 = getelementptr double, double* %t1208, i64 1
  %t1215 = load double, double* %t1214
  %t1216 = bitcast double %t1215 to i64
  %t1217 = alloca i64
  store i64 %t1216, i64* %t1217
  %t1218 = load i64, i64* %t1217
  %t1219 = inttoptr i64 %t1218 to double*
  %t1220 = call double %t1213(double* %t1219, double 1.0e1)
  %t1222 = bitcast double %t1203 to i64
  %t1223 = alloca i64
  store i64 %t1222, i64* %t1223
  %t1224 = load i64, i64* %t1223
  %t1225 = inttoptr i64 %t1224 to i8*
  %t1226 = call i8* @__hulk_num_to_str(double %t1220)
  %t1227 = call i64 @strlen(i8* %t1225)
  %t1228 = call i64 @strlen(i8* %t1226)
  %t1229 = add i64 %t1227, %t1228
  %t1230 = add i64 %t1229, 2
  %t1231 = call i8* @malloc(i64 %t1230)
  call void @__hulk_gc_track(i8* %t1231)
  call i8* @strcpy(i8* %t1231, i8* %t1225)
  call i8* @strcat(i8* %t1231, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1231, i8* %t1226)
  %t1232 = ptrtoint i8* %t1231 to i64
  %t1221 = bitcast i64 %t1232 to double
  %t1233 = bitcast double %t1221 to i64
  %t1234 = alloca i64
  store i64 %t1233, i64* %t1234
  %t1235 = load i64, i64* %t1234
  %t1236 = inttoptr i64 %t1235 to i8*
  call i32 @puts(i8* %t1236)
  %t1238 = ptrtoint i8* getelementptr inbounds ([16 x i8], [16 x i8]* @.slit_1237, i64 0, i64 0) to i64
  %t1239 = bitcast i64 %t1238 to double
  %t1240 = load double, double* %t1200
  %t1241 = bitcast double %t1240 to i64
  %t1242 = alloca i64
  store i64 %t1241, i64* %t1242
  %t1243 = load i64, i64* %t1242
  %t1244 = inttoptr i64 %t1243 to double*
  %t1245 = load double, double* %t1244
  %t1246 = bitcast double %t1245 to i64
  %t1247 = alloca i64
  store i64 %t1246, i64* %t1247
  %t1248 = load i64, i64* %t1247
  %t1249 = inttoptr i64 %t1248 to double (double*, double)*
  %t1250 = getelementptr double, double* %t1244, i64 1
  %t1251 = load double, double* %t1250
  %t1252 = bitcast double %t1251 to i64
  %t1253 = alloca i64
  store i64 %t1252, i64* %t1253
  %t1254 = load i64, i64* %t1253
  %t1255 = inttoptr i64 %t1254 to double*
  %t1256 = call double %t1249(double* %t1255, double 7.0e0)
  %t1258 = bitcast double %t1239 to i64
  %t1259 = alloca i64
  store i64 %t1258, i64* %t1259
  %t1260 = load i64, i64* %t1259
  %t1261 = inttoptr i64 %t1260 to i8*
  %t1262 = call i8* @__hulk_num_to_str(double %t1256)
  %t1263 = call i64 @strlen(i8* %t1261)
  %t1264 = call i64 @strlen(i8* %t1262)
  %t1265 = add i64 %t1263, %t1264
  %t1266 = add i64 %t1265, 2
  %t1267 = call i8* @malloc(i64 %t1266)
  call void @__hulk_gc_track(i8* %t1267)
  call i8* @strcpy(i8* %t1267, i8* %t1261)
  call i8* @strcat(i8* %t1267, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1267, i8* %t1262)
  %t1268 = ptrtoint i8* %t1267 to i64
  %t1257 = bitcast i64 %t1268 to double
  %t1269 = bitcast double %t1257 to i64
  %t1270 = alloca i64
  store i64 %t1269, i64* %t1270
  %t1271 = load i64, i64* %t1270
  %t1272 = inttoptr i64 %t1271 to i8*
  call i32 @puts(i8* %t1272)
  %t1312 = call i8* @malloc(i64 16)
  call void @__hulk_gc_track(i8* %t1312)
  %t1313 = bitcast i8* %t1312 to double*
  %t1314 = ptrtoint double (double*, double)* @__lambda_1273 to i64
  %t1315 = bitcast i64 %t1314 to double
  store double %t1315, double* %t1313
  %t1316 = getelementptr double, double* %t1313, i64 1
  store double 0.0, double* %t1316
  %t1317 = ptrtoint double* %t1313 to i64
  %t1318 = bitcast i64 %t1317 to double
  %t1319 = alloca double
  store double %t1318, double* %t1319
  %t1321 = ptrtoint i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.slit_1320, i64 0, i64 0) to i64
  %t1322 = bitcast i64 %t1321 to double
  %t1323 = load double, double* %t1319
  %t1324 = bitcast double %t1323 to i64
  %t1325 = alloca i64
  store i64 %t1324, i64* %t1325
  %t1326 = load i64, i64* %t1325
  %t1327 = inttoptr i64 %t1326 to double*
  %t1328 = load double, double* %t1327
  %t1329 = bitcast double %t1328 to i64
  %t1330 = alloca i64
  store i64 %t1329, i64* %t1330
  %t1331 = load i64, i64* %t1330
  %t1332 = inttoptr i64 %t1331 to double (double*, double)*
  %t1333 = getelementptr double, double* %t1327, i64 1
  %t1334 = load double, double* %t1333
  %t1335 = bitcast double %t1334 to i64
  %t1336 = alloca i64
  store i64 %t1335, i64* %t1336
  %t1337 = load i64, i64* %t1336
  %t1338 = inttoptr i64 %t1337 to double*
  %t1339 = call double %t1332(double* %t1338, double %t1322)
  %t1340 = bitcast double %t1339 to i64
  %t1341 = alloca i64
  store i64 %t1340, i64* %t1341
  %t1342 = load i64, i64* %t1341
  %t1343 = inttoptr i64 %t1342 to i8*
  call i32 @puts(i8* %t1343)
  %t1345 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_1344, i64 0, i64 0) to i64
  %t1346 = bitcast i64 %t1345 to double
  %t1347 = load double, double* %t1319
  %t1348 = bitcast double %t1347 to i64
  %t1349 = alloca i64
  store i64 %t1348, i64* %t1349
  %t1350 = load i64, i64* %t1349
  %t1351 = inttoptr i64 %t1350 to double*
  %t1352 = load double, double* %t1351
  %t1353 = bitcast double %t1352 to i64
  %t1354 = alloca i64
  store i64 %t1353, i64* %t1354
  %t1355 = load i64, i64* %t1354
  %t1356 = inttoptr i64 %t1355 to double (double*, double)*
  %t1357 = getelementptr double, double* %t1351, i64 1
  %t1358 = load double, double* %t1357
  %t1359 = bitcast double %t1358 to i64
  %t1360 = alloca i64
  store i64 %t1359, i64* %t1360
  %t1361 = load i64, i64* %t1360
  %t1362 = inttoptr i64 %t1361 to double*
  %t1363 = call double %t1356(double* %t1362, double %t1346)
  %t1364 = bitcast double %t1363 to i64
  %t1365 = alloca i64
  store i64 %t1364, i64* %t1365
  %t1366 = load i64, i64* %t1365
  %t1367 = inttoptr i64 %t1366 to i8*
  call i32 @puts(i8* %t1367)
  %t1368 = call double @separator()
  %t1370 = ptrtoint i8* getelementptr inbounds ([23 x i8], [23 x i8]* @.slit_1369, i64 0, i64 0) to i64
  %t1371 = bitcast i64 %t1370 to double
  %t1372 = bitcast double %t1371 to i64
  %t1373 = alloca i64
  store i64 %t1372, i64* %t1373
  %t1374 = load i64, i64* %t1373
  %t1375 = inttoptr i64 %t1374 to i8*
  call i32 @puts(i8* %t1375)
  %t1377 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_1376, i64 0, i64 0) to i64
  %t1378 = bitcast i64 %t1377 to double
  %t1380 = bitcast double %t1378 to i64
  %t1381 = alloca i64
  store i64 %t1380, i64* %t1381
  %t1382 = load i64, i64* %t1381
  %t1383 = inttoptr i64 %t1382 to i8*
  %t1384 = call i8* @__hulk_num_to_str(double 3.141592653589793e0)
  %t1385 = call i64 @strlen(i8* %t1383)
  %t1386 = call i64 @strlen(i8* %t1384)
  %t1387 = add i64 %t1385, %t1386
  %t1388 = add i64 %t1387, 2
  %t1389 = call i8* @malloc(i64 %t1388)
  call void @__hulk_gc_track(i8* %t1389)
  call i8* @strcpy(i8* %t1389, i8* %t1383)
  call i8* @strcat(i8* %t1389, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1389, i8* %t1384)
  %t1390 = ptrtoint i8* %t1389 to i64
  %t1379 = bitcast i64 %t1390 to double
  %t1391 = bitcast double %t1379 to i64
  %t1392 = alloca i64
  store i64 %t1391, i64* %t1392
  %t1393 = load i64, i64* %t1392
  %t1394 = inttoptr i64 %t1393 to i8*
  call i32 @puts(i8* %t1394)
  %t1396 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_1395, i64 0, i64 0) to i64
  %t1397 = bitcast i64 %t1396 to double
  %t1399 = bitcast double %t1397 to i64
  %t1400 = alloca i64
  store i64 %t1399, i64* %t1400
  %t1401 = load i64, i64* %t1400
  %t1402 = inttoptr i64 %t1401 to i8*
  %t1403 = call i8* @__hulk_num_to_str(double 2.718281828459045e0)
  %t1404 = call i64 @strlen(i8* %t1402)
  %t1405 = call i64 @strlen(i8* %t1403)
  %t1406 = add i64 %t1404, %t1405
  %t1407 = add i64 %t1406, 2
  %t1408 = call i8* @malloc(i64 %t1407)
  call void @__hulk_gc_track(i8* %t1408)
  call i8* @strcpy(i8* %t1408, i8* %t1402)
  call i8* @strcat(i8* %t1408, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1408, i8* %t1403)
  %t1409 = ptrtoint i8* %t1408 to i64
  %t1398 = bitcast i64 %t1409 to double
  %t1410 = bitcast double %t1398 to i64
  %t1411 = alloca i64
  store i64 %t1410, i64* %t1411
  %t1412 = load i64, i64* %t1411
  %t1413 = inttoptr i64 %t1412 to i8*
  call i32 @puts(i8* %t1413)
  %t1415 = ptrtoint i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.slit_1414, i64 0, i64 0) to i64
  %t1416 = bitcast i64 %t1415 to double
  %t1418 = bitcast double %t1416 to i64
  %t1419 = alloca i64
  store i64 %t1418, i64* %t1419
  %t1420 = load i64, i64* %t1419
  %t1421 = inttoptr i64 %t1420 to i8*
  %t1422 = call i8* @__hulk_num_to_str(double 1.2e1)
  %t1423 = call i64 @strlen(i8* %t1421)
  %t1424 = call i64 @strlen(i8* %t1422)
  %t1425 = add i64 %t1423, %t1424
  %t1426 = add i64 %t1425, 2
  %t1427 = call i8* @malloc(i64 %t1426)
  call void @__hulk_gc_track(i8* %t1427)
  call i8* @strcpy(i8* %t1427, i8* %t1421)
  call i8* @strcat(i8* %t1427, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1427, i8* %t1422)
  %t1428 = ptrtoint i8* %t1427 to i64
  %t1417 = bitcast i64 %t1428 to double
  %t1429 = bitcast double %t1417 to i64
  %t1430 = alloca i64
  store i64 %t1429, i64* %t1430
  %t1431 = load i64, i64* %t1430
  %t1432 = inttoptr i64 %t1431 to i8*
  call i32 @puts(i8* %t1432)
  %t1434 = ptrtoint i8* getelementptr inbounds ([9 x i8], [9 x i8]* @.slit_1433, i64 0, i64 0) to i64
  %t1435 = bitcast i64 %t1434 to double
  %t1437 = bitcast double %t1435 to i64
  %t1438 = alloca i64
  store i64 %t1437, i64* %t1438
  %t1439 = load i64, i64* %t1438
  %t1440 = inttoptr i64 %t1439 to i8*
  %t1441 = call i8* @__hulk_num_to_str(double 0.0e0)
  %t1442 = call i64 @strlen(i8* %t1440)
  %t1443 = call i64 @strlen(i8* %t1441)
  %t1444 = add i64 %t1442, %t1443
  %t1445 = add i64 %t1444, 2
  %t1446 = call i8* @malloc(i64 %t1445)
  call void @__hulk_gc_track(i8* %t1446)
  call i8* @strcpy(i8* %t1446, i8* %t1440)
  call i8* @strcat(i8* %t1446, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1446, i8* %t1441)
  %t1447 = ptrtoint i8* %t1446 to i64
  %t1436 = bitcast i64 %t1447 to double
  %t1448 = bitcast double %t1436 to i64
  %t1449 = alloca i64
  store i64 %t1448, i64* %t1449
  %t1450 = load i64, i64* %t1449
  %t1451 = inttoptr i64 %t1450 to i8*
  call i32 @puts(i8* %t1451)
  %t1453 = ptrtoint i8* getelementptr inbounds ([9 x i8], [9 x i8]* @.slit_1452, i64 0, i64 0) to i64
  %t1454 = bitcast i64 %t1453 to double
  %t1456 = bitcast double %t1454 to i64
  %t1457 = alloca i64
  store i64 %t1456, i64* %t1457
  %t1458 = load i64, i64* %t1457
  %t1459 = inttoptr i64 %t1458 to i8*
  %t1460 = call i8* @__hulk_num_to_str(double 1.0e0)
  %t1461 = call i64 @strlen(i8* %t1459)
  %t1462 = call i64 @strlen(i8* %t1460)
  %t1463 = add i64 %t1461, %t1462
  %t1464 = add i64 %t1463, 2
  %t1465 = call i8* @malloc(i64 %t1464)
  call void @__hulk_gc_track(i8* %t1465)
  call i8* @strcpy(i8* %t1465, i8* %t1459)
  call i8* @strcat(i8* %t1465, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1465, i8* %t1460)
  %t1466 = ptrtoint i8* %t1465 to i64
  %t1455 = bitcast i64 %t1466 to double
  %t1467 = bitcast double %t1455 to i64
  %t1468 = alloca i64
  store i64 %t1467, i64* %t1468
  %t1469 = load i64, i64* %t1468
  %t1470 = inttoptr i64 %t1469 to i8*
  call i32 @puts(i8* %t1470)
  %t1472 = ptrtoint i8* getelementptr inbounds ([9 x i8], [9 x i8]* @.slit_1471, i64 0, i64 0) to i64
  %t1473 = bitcast i64 %t1472 to double
  %t1475 = bitcast double %t1473 to i64
  %t1476 = alloca i64
  store i64 %t1475, i64* %t1476
  %t1477 = load i64, i64* %t1476
  %t1478 = inttoptr i64 %t1477 to i8*
  %t1479 = call i8* @__hulk_num_to_str(double 2.718281828459045e0)
  %t1480 = call i64 @strlen(i8* %t1478)
  %t1481 = call i64 @strlen(i8* %t1479)
  %t1482 = add i64 %t1480, %t1481
  %t1483 = add i64 %t1482, 2
  %t1484 = call i8* @malloc(i64 %t1483)
  call void @__hulk_gc_track(i8* %t1484)
  call i8* @strcpy(i8* %t1484, i8* %t1478)
  call i8* @strcat(i8* %t1484, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1484, i8* %t1479)
  %t1485 = ptrtoint i8* %t1484 to i64
  %t1474 = bitcast i64 %t1485 to double
  %t1486 = bitcast double %t1474 to i64
  %t1487 = alloca i64
  store i64 %t1486, i64* %t1487
  %t1488 = load i64, i64* %t1487
  %t1489 = inttoptr i64 %t1488 to i8*
  call i32 @puts(i8* %t1489)
  %t1491 = ptrtoint i8* getelementptr inbounds ([16 x i8], [16 x i8]* @.slit_1490, i64 0, i64 0) to i64
  %t1492 = bitcast i64 %t1491 to double
  %t1494 = bitcast double %t1492 to i64
  %t1495 = alloca i64
  store i64 %t1494, i64* %t1495
  %t1496 = load i64, i64* %t1495
  %t1497 = inttoptr i64 %t1496 to i8*
  %t1498 = call i8* @__hulk_num_to_str(double 2.9999999999999996e0)
  %t1499 = call i64 @strlen(i8* %t1497)
  %t1500 = call i64 @strlen(i8* %t1498)
  %t1501 = add i64 %t1499, %t1500
  %t1502 = add i64 %t1501, 2
  %t1503 = call i8* @malloc(i64 %t1502)
  call void @__hulk_gc_track(i8* %t1503)
  call i8* @strcpy(i8* %t1503, i8* %t1497)
  call i8* @strcat(i8* %t1503, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1503, i8* %t1498)
  %t1504 = ptrtoint i8* %t1503 to i64
  %t1493 = bitcast i64 %t1504 to double
  %t1505 = bitcast double %t1493 to i64
  %t1506 = alloca i64
  store i64 %t1505, i64* %t1506
  %t1507 = load i64, i64* %t1506
  %t1508 = inttoptr i64 %t1507 to i8*
  call i32 @puts(i8* %t1508)
  %t1509 = load i1, i1* @.rand_seeded
  br i1 %t1509, label %rand_call_1511, label %rand_seed_1510
rand_seed_1510:
  %t1512 = call i64 @time(i64* null)
  %t1513 = trunc i64 %t1512 to i32
  call void @srand(i32 %t1513)
  store i1 true, i1* @.rand_seeded
  br label %rand_call_1511
rand_call_1511:
  %t1514 = call i32 @rand()
  %t1515 = sitofp i32 %t1514 to double
  %t1516 = fdiv double %t1515, 2.147483647e9
  %t1517 = alloca double
  store double %t1516, double* %t1517
  %t1519 = ptrtoint i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.slit_1518, i64 0, i64 0) to i64
  %t1520 = bitcast i64 %t1519 to double
  %t1521 = bitcast double %t1520 to i64
  %t1522 = alloca i64
  store i64 %t1521, i64* %t1522
  %t1523 = load i64, i64* %t1522
  %t1524 = inttoptr i64 %t1523 to i8*
  call i32 @puts(i8* %t1524)
  %t1525 = load double, double* %t1517
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double %t1525)
  call i32 @puts(i8* getelementptr inbounds ([1 x i8], [1 x i8]* @.empty_s, i64 0, i64 0))
  %t1526 = call double @separator()
  %t1528 = ptrtoint i8* getelementptr inbounds ([31 x i8], [31 x i8]* @.slit_1527, i64 0, i64 0) to i64
  %t1529 = bitcast i64 %t1528 to double
  %t1530 = bitcast double %t1529 to i64
  %t1531 = alloca i64
  store i64 %t1530, i64* %t1531
  %t1532 = load i64, i64* %t1531
  %t1533 = inttoptr i64 %t1532 to i8*
  call i32 @puts(i8* %t1533)
  %t1534 = alloca double
  store double 1.0e1, double* %t1534
  %t1535 = alloca double
  store double 0.0e0, double* %t1535
  %t1536 = alloca double
  store double 1.0e0, double* %t1536
  %t1537 = alloca double
  store double 0.0e0, double* %t1537
  br label %wcond_1538
wcond_1538:
  %t1541 = load double, double* %t1537
  %t1543 = fcmp olt double %t1541, 1.0e1
  %t1542 = uitofp i1 %t1543 to double
  %t1544 = fcmp one double %t1542, 0.0
  br i1 %t1544, label %wbody_1539, label %wend_1540
wbody_1539:
  %t1545 = load double, double* %t1535
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double %t1545)
  call i32 @puts(i8* getelementptr inbounds ([1 x i8], [1 x i8]* @.empty_s, i64 0, i64 0))
  %t1546 = load double, double* %t1536
  %t1547 = alloca double
  store double %t1546, double* %t1547
  %t1548 = load double, double* %t1535
  %t1549 = load double, double* %t1536
  %t1550 = fadd double %t1548, %t1549
  store double %t1550, double* %t1536
  %t1551 = load double, double* %t1547
  store double %t1551, double* %t1535
  %t1552 = load double, double* %t1537
  %t1553 = fadd double %t1552, 1.0e0
  store double %t1553, double* %t1537
  br label %wcond_1538
wend_1540:
  %t1554 = call double @separator()
  %t1556 = ptrtoint i8* getelementptr inbounds ([26 x i8], [26 x i8]* @.slit_1555, i64 0, i64 0) to i64
  %t1557 = bitcast i64 %t1556 to double
  %t1558 = bitcast double %t1557 to i64
  %t1559 = alloca i64
  store i64 %t1558, i64* %t1559
  %t1560 = load i64, i64* %t1559
  %t1561 = inttoptr i64 %t1560 to i8*
  call i32 @puts(i8* %t1561)
  %t1562 = alloca double
  store double 0.0e0, double* %t1562
  %t1563 = alloca double
  store double 1.0e0, double* %t1563
  br label %wcond_1564
wcond_1564:
  %t1567 = load double, double* %t1562
  %t1569 = fcmp olt double %t1567, 5.0e0
  %t1568 = uitofp i1 %t1569 to double
  %t1570 = fcmp one double %t1568, 0.0
  br i1 %t1570, label %wbody_1565, label %wend_1566
wbody_1565:
  %t1572 = ptrtoint i8* getelementptr inbounds ([10 x i8], [10 x i8]* @.slit_1571, i64 0, i64 0) to i64
  %t1573 = bitcast i64 %t1572 to double
  %t1574 = load double, double* %t1562
  %t1576 = bitcast double %t1573 to i64
  %t1577 = alloca i64
  store i64 %t1576, i64* %t1577
  %t1578 = load i64, i64* %t1577
  %t1579 = inttoptr i64 %t1578 to i8*
  %t1580 = call i8* @__hulk_num_to_str(double %t1574)
  %t1581 = call i64 @strlen(i8* %t1579)
  %t1582 = call i64 @strlen(i8* %t1580)
  %t1583 = add i64 %t1581, %t1582
  %t1584 = add i64 %t1583, 2
  %t1585 = call i8* @malloc(i64 %t1584)
  call void @__hulk_gc_track(i8* %t1585)
  call i8* @strcpy(i8* %t1585, i8* %t1579)
  call i8* @strcat(i8* %t1585, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1585, i8* %t1580)
  %t1586 = ptrtoint i8* %t1585 to i64
  %t1575 = bitcast i64 %t1586 to double
  %t1587 = bitcast double %t1575 to i64
  %t1588 = alloca i64
  store i64 %t1587, i64* %t1588
  %t1589 = load i64, i64* %t1588
  %t1590 = inttoptr i64 %t1589 to i8*
  call i32 @puts(i8* %t1590)
  %t1591 = load double, double* %t1562
  %t1592 = load double, double* %t1563
  %t1593 = fadd double %t1591, %t1592
  store double %t1593, double* %t1562
  %t1594 = load double, double* %t1563
  %t1595 = fadd double %t1594, 1.0e0
  store double %t1595, double* %t1563
  br label %wcond_1564
wend_1566:
  %t1597 = ptrtoint i8* getelementptr inbounds ([16 x i8], [16 x i8]* @.slit_1596, i64 0, i64 0) to i64
  %t1598 = bitcast i64 %t1597 to double
  %t1599 = load double, double* %t1562
  %t1601 = bitcast double %t1598 to i64
  %t1602 = alloca i64
  store i64 %t1601, i64* %t1602
  %t1603 = load i64, i64* %t1602
  %t1604 = inttoptr i64 %t1603 to i8*
  %t1605 = call i8* @__hulk_num_to_str(double %t1599)
  %t1606 = call i64 @strlen(i8* %t1604)
  %t1607 = call i64 @strlen(i8* %t1605)
  %t1608 = add i64 %t1606, %t1607
  %t1609 = add i64 %t1608, 2
  %t1610 = call i8* @malloc(i64 %t1609)
  call void @__hulk_gc_track(i8* %t1610)
  call i8* @strcpy(i8* %t1610, i8* %t1604)
  call i8* @strcat(i8* %t1610, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1610, i8* %t1605)
  %t1611 = ptrtoint i8* %t1610 to i64
  %t1600 = bitcast i64 %t1611 to double
  %t1612 = bitcast double %t1600 to i64
  %t1613 = alloca i64
  store i64 %t1612, i64* %t1613
  %t1614 = load i64, i64* %t1613
  %t1615 = inttoptr i64 %t1614 to i8*
  call i32 @puts(i8* %t1615)
  %t1616 = call double @separator()
  %t1618 = ptrtoint i8* getelementptr inbounds ([26 x i8], [26 x i8]* @.slit_1617, i64 0, i64 0) to i64
  %t1619 = bitcast i64 %t1618 to double
  %t1620 = bitcast double %t1619 to i64
  %t1621 = alloca i64
  store i64 %t1620, i64* %t1621
  %t1622 = load i64, i64* %t1621
  %t1623 = inttoptr i64 %t1622 to i8*
  call i32 @puts(i8* %t1623)
  %t1625 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_1624, i64 0, i64 0) to i64
  %t1626 = bitcast i64 %t1625 to double
  %t1627 = alloca double
  store double %t1626, double* %t1627
  %t1629 = ptrtoint i8* getelementptr inbounds ([9 x i8], [9 x i8]* @.slit_1628, i64 0, i64 0) to i64
  %t1630 = bitcast i64 %t1629 to double
  %t1631 = alloca double
  store double %t1630, double* %t1631
  %t1633 = ptrtoint i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.slit_1632, i64 0, i64 0) to i64
  %t1634 = bitcast i64 %t1633 to double
  %t1635 = alloca double
  store double %t1634, double* %t1635
  %t1637 = ptrtoint i8* getelementptr inbounds ([19 x i8], [19 x i8]* @.slit_1636, i64 0, i64 0) to i64
  %t1638 = bitcast i64 %t1637 to double
  %t1639 = bitcast double %t1638 to i64
  %t1640 = alloca i64
  store i64 %t1639, i64* %t1640
  %t1641 = load i64, i64* %t1640
  %t1642 = inttoptr i64 %t1641 to i8*
  call i32 @puts(i8* %t1642)
  %t1644 = ptrtoint i8* getelementptr inbounds ([18 x i8], [18 x i8]* @.slit_1643, i64 0, i64 0) to i64
  %t1645 = bitcast i64 %t1644 to double
  %t1646 = bitcast double %t1645 to i64
  %t1647 = alloca i64
  store i64 %t1646, i64* %t1647
  %t1648 = load i64, i64* %t1647
  %t1649 = inttoptr i64 %t1648 to i8*
  call i32 @puts(i8* %t1649)
  %t1651 = ptrtoint i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.slit_1650, i64 0, i64 0) to i64
  %t1652 = bitcast i64 %t1651 to double
  %t1653 = call double @repeat_str(double %t1652, double 1.0e1)
  %t1654 = alloca double
  store double %t1653, double* %t1654
  %t1655 = load double, double* %t1654
  %t1657 = ptrtoint i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.slit_1656, i64 0, i64 0) to i64
  %t1658 = bitcast i64 %t1657 to double
  %t1660 = bitcast double %t1655 to i64
  %t1661 = alloca i64
  store i64 %t1660, i64* %t1661
  %t1662 = load i64, i64* %t1661
  %t1663 = inttoptr i64 %t1662 to i8*
  %t1664 = bitcast double %t1658 to i64
  %t1665 = alloca i64
  store i64 %t1664, i64* %t1665
  %t1666 = load i64, i64* %t1665
  %t1667 = inttoptr i64 %t1666 to i8*
  %t1668 = call i64 @strlen(i8* %t1663)
  %t1669 = call i64 @strlen(i8* %t1667)
  %t1670 = add i64 %t1668, %t1669
  %t1671 = add i64 %t1670, 1
  %t1672 = call i8* @malloc(i64 %t1671)
  call void @__hulk_gc_track(i8* %t1672)
  call i8* @strcpy(i8* %t1672, i8* %t1663)
  call i8* @strcat(i8* %t1672, i8* %t1667)
  %t1673 = ptrtoint i8* %t1672 to i64
  %t1659 = bitcast i64 %t1673 to double
  %t1675 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_1674, i64 0, i64 0) to i64
  %t1676 = bitcast i64 %t1675 to double
  %t1678 = bitcast double %t1659 to i64
  %t1679 = alloca i64
  store i64 %t1678, i64* %t1679
  %t1680 = load i64, i64* %t1679
  %t1681 = inttoptr i64 %t1680 to i8*
  %t1682 = bitcast double %t1676 to i64
  %t1683 = alloca i64
  store i64 %t1682, i64* %t1683
  %t1684 = load i64, i64* %t1683
  %t1685 = inttoptr i64 %t1684 to i8*
  %t1686 = call i64 @strlen(i8* %t1681)
  %t1687 = call i64 @strlen(i8* %t1685)
  %t1688 = add i64 %t1686, %t1687
  %t1689 = add i64 %t1688, 1
  %t1690 = call i8* @malloc(i64 %t1689)
  call void @__hulk_gc_track(i8* %t1690)
  call i8* @strcpy(i8* %t1690, i8* %t1681)
  call i8* @strcat(i8* %t1690, i8* %t1685)
  %t1691 = ptrtoint i8* %t1690 to i64
  %t1677 = bitcast i64 %t1691 to double
  %t1693 = ptrtoint i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.slit_1692, i64 0, i64 0) to i64
  %t1694 = bitcast i64 %t1693 to double
  %t1696 = bitcast double %t1677 to i64
  %t1697 = alloca i64
  store i64 %t1696, i64* %t1697
  %t1698 = load i64, i64* %t1697
  %t1699 = inttoptr i64 %t1698 to i8*
  %t1700 = bitcast double %t1694 to i64
  %t1701 = alloca i64
  store i64 %t1700, i64* %t1701
  %t1702 = load i64, i64* %t1701
  %t1703 = inttoptr i64 %t1702 to i8*
  %t1704 = call i64 @strlen(i8* %t1699)
  %t1705 = call i64 @strlen(i8* %t1703)
  %t1706 = add i64 %t1704, %t1705
  %t1707 = add i64 %t1706, 1
  %t1708 = call i8* @malloc(i64 %t1707)
  call void @__hulk_gc_track(i8* %t1708)
  call i8* @strcpy(i8* %t1708, i8* %t1699)
  call i8* @strcat(i8* %t1708, i8* %t1703)
  %t1709 = ptrtoint i8* %t1708 to i64
  %t1695 = bitcast i64 %t1709 to double
  %t1710 = load double, double* %t1654
  %t1712 = bitcast double %t1695 to i64
  %t1713 = alloca i64
  store i64 %t1712, i64* %t1713
  %t1714 = load i64, i64* %t1713
  %t1715 = inttoptr i64 %t1714 to i8*
  %t1716 = bitcast double %t1710 to i64
  %t1717 = alloca i64
  store i64 %t1716, i64* %t1717
  %t1718 = load i64, i64* %t1717
  %t1719 = inttoptr i64 %t1718 to i8*
  %t1720 = call i64 @strlen(i8* %t1715)
  %t1721 = call i64 @strlen(i8* %t1719)
  %t1722 = add i64 %t1720, %t1721
  %t1723 = add i64 %t1722, 1
  %t1724 = call i8* @malloc(i64 %t1723)
  call void @__hulk_gc_track(i8* %t1724)
  call i8* @strcpy(i8* %t1724, i8* %t1715)
  call i8* @strcat(i8* %t1724, i8* %t1719)
  %t1725 = ptrtoint i8* %t1724 to i64
  %t1711 = bitcast i64 %t1725 to double
  %t1726 = bitcast double %t1711 to i64
  %t1727 = alloca i64
  store i64 %t1726, i64* %t1727
  %t1728 = load i64, i64* %t1727
  %t1729 = inttoptr i64 %t1728 to i8*
  call i32 @puts(i8* %t1729)
  %t1730 = call double @separator()
  %t1732 = ptrtoint i8* getelementptr inbounds ([26 x i8], [26 x i8]* @.slit_1731, i64 0, i64 0) to i64
  %t1733 = bitcast i64 %t1732 to double
  %t1734 = bitcast double %t1733 to i64
  %t1735 = alloca i64
  store i64 %t1734, i64* %t1735
  %t1736 = load i64, i64* %t1735
  %t1737 = inttoptr i64 %t1736 to i8*
  call i32 @puts(i8* %t1737)
  %t1739 = ptrtoint i8* getelementptr inbounds ([11 x i8], [11 x i8]* @.slit_1738, i64 0, i64 0) to i64
  %t1740 = bitcast i64 %t1739 to double
  %t1741 = call double @abs(double -4.2e1)
  %t1743 = bitcast double %t1740 to i64
  %t1744 = alloca i64
  store i64 %t1743, i64* %t1744
  %t1745 = load i64, i64* %t1744
  %t1746 = inttoptr i64 %t1745 to i8*
  %t1747 = call i8* @__hulk_num_to_str(double %t1741)
  %t1748 = call i64 @strlen(i8* %t1746)
  %t1749 = call i64 @strlen(i8* %t1747)
  %t1750 = add i64 %t1748, %t1749
  %t1751 = add i64 %t1750, 2
  %t1752 = call i8* @malloc(i64 %t1751)
  call void @__hulk_gc_track(i8* %t1752)
  call i8* @strcpy(i8* %t1752, i8* %t1746)
  call i8* @strcat(i8* %t1752, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1752, i8* %t1747)
  %t1753 = ptrtoint i8* %t1752 to i64
  %t1742 = bitcast i64 %t1753 to double
  %t1754 = bitcast double %t1742 to i64
  %t1755 = alloca i64
  store i64 %t1754, i64* %t1755
  %t1756 = load i64, i64* %t1755
  %t1757 = inttoptr i64 %t1756 to i8*
  call i32 @puts(i8* %t1757)
  %t1759 = ptrtoint i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.slit_1758, i64 0, i64 0) to i64
  %t1760 = bitcast i64 %t1759 to double
  %t1761 = call double @max(double 1.0e1, double 2.0e1)
  %t1763 = bitcast double %t1760 to i64
  %t1764 = alloca i64
  store i64 %t1763, i64* %t1764
  %t1765 = load i64, i64* %t1764
  %t1766 = inttoptr i64 %t1765 to i8*
  %t1767 = call i8* @__hulk_num_to_str(double %t1761)
  %t1768 = call i64 @strlen(i8* %t1766)
  %t1769 = call i64 @strlen(i8* %t1767)
  %t1770 = add i64 %t1768, %t1769
  %t1771 = add i64 %t1770, 2
  %t1772 = call i8* @malloc(i64 %t1771)
  call void @__hulk_gc_track(i8* %t1772)
  call i8* @strcpy(i8* %t1772, i8* %t1766)
  call i8* @strcat(i8* %t1772, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1772, i8* %t1767)
  %t1773 = ptrtoint i8* %t1772 to i64
  %t1762 = bitcast i64 %t1773 to double
  %t1774 = bitcast double %t1762 to i64
  %t1775 = alloca i64
  store i64 %t1774, i64* %t1775
  %t1776 = load i64, i64* %t1775
  %t1777 = inttoptr i64 %t1776 to i8*
  call i32 @puts(i8* %t1777)
  %t1779 = ptrtoint i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.slit_1778, i64 0, i64 0) to i64
  %t1780 = bitcast i64 %t1779 to double
  %t1781 = call double @min(double 1.0e1, double 2.0e1)
  %t1783 = bitcast double %t1780 to i64
  %t1784 = alloca i64
  store i64 %t1783, i64* %t1784
  %t1785 = load i64, i64* %t1784
  %t1786 = inttoptr i64 %t1785 to i8*
  %t1787 = call i8* @__hulk_num_to_str(double %t1781)
  %t1788 = call i64 @strlen(i8* %t1786)
  %t1789 = call i64 @strlen(i8* %t1787)
  %t1790 = add i64 %t1788, %t1789
  %t1791 = add i64 %t1790, 2
  %t1792 = call i8* @malloc(i64 %t1791)
  call void @__hulk_gc_track(i8* %t1792)
  call i8* @strcpy(i8* %t1792, i8* %t1786)
  call i8* @strcat(i8* %t1792, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1792, i8* %t1787)
  %t1793 = ptrtoint i8* %t1792 to i64
  %t1782 = bitcast i64 %t1793 to double
  %t1794 = bitcast double %t1782 to i64
  %t1795 = alloca i64
  store i64 %t1794, i64* %t1795
  %t1796 = load i64, i64* %t1795
  %t1797 = inttoptr i64 %t1796 to i8*
  call i32 @puts(i8* %t1797)
  %t1799 = ptrtoint i8* getelementptr inbounds ([21 x i8], [21 x i8]* @.slit_1798, i64 0, i64 0) to i64
  %t1800 = bitcast i64 %t1799 to double
  %t1801 = call double @clamp(double 1.5e2, double 0.0e0, double 1.0e2)
  %t1803 = bitcast double %t1800 to i64
  %t1804 = alloca i64
  store i64 %t1803, i64* %t1804
  %t1805 = load i64, i64* %t1804
  %t1806 = inttoptr i64 %t1805 to i8*
  %t1807 = call i8* @__hulk_num_to_str(double %t1801)
  %t1808 = call i64 @strlen(i8* %t1806)
  %t1809 = call i64 @strlen(i8* %t1807)
  %t1810 = add i64 %t1808, %t1809
  %t1811 = add i64 %t1810, 2
  %t1812 = call i8* @malloc(i64 %t1811)
  call void @__hulk_gc_track(i8* %t1812)
  call i8* @strcpy(i8* %t1812, i8* %t1806)
  call i8* @strcat(i8* %t1812, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1812, i8* %t1807)
  %t1813 = ptrtoint i8* %t1812 to i64
  %t1802 = bitcast i64 %t1813 to double
  %t1814 = bitcast double %t1802 to i64
  %t1815 = alloca i64
  store i64 %t1814, i64* %t1815
  %t1816 = load i64, i64* %t1815
  %t1817 = inttoptr i64 %t1816 to i8*
  call i32 @puts(i8* %t1817)
  %t1819 = ptrtoint i8* getelementptr inbounds ([20 x i8], [20 x i8]* @.slit_1818, i64 0, i64 0) to i64
  %t1820 = bitcast i64 %t1819 to double
  %t1821 = call double @clamp(double -5.0e0, double 0.0e0, double 1.0e2)
  %t1823 = bitcast double %t1820 to i64
  %t1824 = alloca i64
  store i64 %t1823, i64* %t1824
  %t1825 = load i64, i64* %t1824
  %t1826 = inttoptr i64 %t1825 to i8*
  %t1827 = call i8* @__hulk_num_to_str(double %t1821)
  %t1828 = call i64 @strlen(i8* %t1826)
  %t1829 = call i64 @strlen(i8* %t1827)
  %t1830 = add i64 %t1828, %t1829
  %t1831 = add i64 %t1830, 2
  %t1832 = call i8* @malloc(i64 %t1831)
  call void @__hulk_gc_track(i8* %t1832)
  call i8* @strcpy(i8* %t1832, i8* %t1826)
  call i8* @strcat(i8* %t1832, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1832, i8* %t1827)
  %t1833 = ptrtoint i8* %t1832 to i64
  %t1822 = bitcast i64 %t1833 to double
  %t1834 = bitcast double %t1822 to i64
  %t1835 = alloca i64
  store i64 %t1834, i64* %t1835
  %t1836 = load i64, i64* %t1835
  %t1837 = inttoptr i64 %t1836 to i8*
  call i32 @puts(i8* %t1837)
  %t1838 = call double @separator()
  %t1840 = ptrtoint i8* getelementptr inbounds ([22 x i8], [22 x i8]* @.slit_1839, i64 0, i64 0) to i64
  %t1841 = bitcast i64 %t1840 to double
  %t1842 = bitcast double %t1841 to i64
  %t1843 = alloca i64
  store i64 %t1842, i64* %t1843
  %t1844 = load i64, i64* %t1843
  %t1845 = inttoptr i64 %t1844 to i8*
  call i32 @puts(i8* %t1845)
  %t1847 = ptrtoint i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.slit_1846, i64 0, i64 0) to i64
  %t1848 = bitcast i64 %t1847 to double
  %t1849 = bitcast double %t1848 to i64
  %t1850 = alloca i64
  store i64 %t1849, i64* %t1850
  %t1851 = load i64, i64* %t1850
  %t1852 = inttoptr i64 %t1851 to i8*
  call i32 @puts(i8* %t1852)
  %t1853 = load double, double* %t640
  %t1854 = bitcast double %t1853 to i64
  %t1855 = alloca i64
  store i64 %t1854, i64* %t1855
  %t1856 = load i64, i64* %t1855
  %t1857 = inttoptr i64 %t1856 to i8*
  %t1858 = call double @Animal_is_quadruped(i8* %t1857)
  %t1859 = fcmp one double %t1858, 0.0
  %t1860 = select i1 %t1859, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t1860)
  %t1862 = ptrtoint i8* getelementptr inbounds ([17 x i8], [17 x i8]* @.slit_1861, i64 0, i64 0) to i64
  %t1863 = bitcast i64 %t1862 to double
  %t1864 = bitcast double %t1863 to i64
  %t1865 = alloca i64
  store i64 %t1864, i64* %t1865
  %t1866 = load i64, i64* %t1865
  %t1867 = inttoptr i64 %t1866 to i8*
  call i32 @puts(i8* %t1867)
  %t1868 = load double, double* %t671
  %t1869 = bitcast double %t1868 to i64
  %t1870 = alloca i64
  store i64 %t1869, i64* %t1870
  %t1871 = load i64, i64* %t1870
  %t1872 = inttoptr i64 %t1871 to i8*
  %t1873 = call double @Animal_is_quadruped(i8* %t1872)
  %t1874 = fcmp one double %t1873, 0.0
  %t1875 = select i1 %t1874, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t1875)
  %t1876 = call double @separator()
  %t1878 = ptrtoint i8* getelementptr inbounds ([21 x i8], [21 x i8]* @.slit_1877, i64 0, i64 0) to i64
  %t1879 = bitcast i64 %t1878 to double
  %t1880 = bitcast double %t1879 to i64
  %t1881 = alloca i64
  store i64 %t1880, i64* %t1881
  %t1882 = load i64, i64* %t1881
  %t1883 = inttoptr i64 %t1882 to i8*
  call i32 @puts(i8* %t1883)
  %t1885 = ptrtoint i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.slit_1884, i64 0, i64 0) to i64
  %t1886 = bitcast i64 %t1885 to double
  %t1888 = bitcast double %t1886 to i64
  %t1889 = alloca i64
  store i64 %t1888, i64* %t1889
  %t1890 = load i64, i64* %t1889
  %t1891 = inttoptr i64 %t1890 to i8*
  %t1892 = call i8* @__hulk_num_to_str(double 1.4e1)
  %t1893 = call i64 @strlen(i8* %t1891)
  %t1894 = call i64 @strlen(i8* %t1892)
  %t1895 = add i64 %t1893, %t1894
  %t1896 = add i64 %t1895, 2
  %t1897 = call i8* @malloc(i64 %t1896)
  call void @__hulk_gc_track(i8* %t1897)
  call i8* @strcpy(i8* %t1897, i8* %t1891)
  call i8* @strcat(i8* %t1897, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1897, i8* %t1892)
  %t1898 = ptrtoint i8* %t1897 to i64
  %t1887 = bitcast i64 %t1898 to double
  %t1899 = bitcast double %t1887 to i64
  %t1900 = alloca i64
  store i64 %t1899, i64* %t1900
  %t1901 = load i64, i64* %t1900
  %t1902 = inttoptr i64 %t1901 to i8*
  call i32 @puts(i8* %t1902)
  %t1904 = ptrtoint i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.slit_1903, i64 0, i64 0) to i64
  %t1905 = bitcast i64 %t1904 to double
  %t1907 = bitcast double %t1905 to i64
  %t1908 = alloca i64
  store i64 %t1907, i64* %t1908
  %t1909 = load i64, i64* %t1908
  %t1910 = inttoptr i64 %t1909 to i8*
  %t1911 = call i8* @__hulk_num_to_str(double 1.5e1)
  %t1912 = call i64 @strlen(i8* %t1910)
  %t1913 = call i64 @strlen(i8* %t1911)
  %t1914 = add i64 %t1912, %t1913
  %t1915 = add i64 %t1914, 2
  %t1916 = call i8* @malloc(i64 %t1915)
  call void @__hulk_gc_track(i8* %t1916)
  call i8* @strcpy(i8* %t1916, i8* %t1910)
  call i8* @strcat(i8* %t1916, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1916, i8* %t1911)
  %t1917 = ptrtoint i8* %t1916 to i64
  %t1906 = bitcast i64 %t1917 to double
  %t1918 = bitcast double %t1906 to i64
  %t1919 = alloca i64
  store i64 %t1918, i64* %t1919
  %t1920 = load i64, i64* %t1919
  %t1921 = inttoptr i64 %t1920 to i8*
  call i32 @puts(i8* %t1921)
  %t1923 = ptrtoint i8* getelementptr inbounds ([20 x i8], [20 x i8]* @.slit_1922, i64 0, i64 0) to i64
  %t1924 = bitcast i64 %t1923 to double
  %t1926 = bitcast double %t1924 to i64
  %t1927 = alloca i64
  store i64 %t1926, i64* %t1927
  %t1928 = load i64, i64* %t1927
  %t1929 = inttoptr i64 %t1928 to i8*
  %t1930 = call i8* @__hulk_num_to_str(double 1.8e1)
  %t1931 = call i64 @strlen(i8* %t1929)
  %t1932 = call i64 @strlen(i8* %t1930)
  %t1933 = add i64 %t1931, %t1932
  %t1934 = add i64 %t1933, 2
  %t1935 = call i8* @malloc(i64 %t1934)
  call void @__hulk_gc_track(i8* %t1935)
  call i8* @strcpy(i8* %t1935, i8* %t1929)
  call i8* @strcat(i8* %t1935, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1935, i8* %t1930)
  %t1936 = ptrtoint i8* %t1935 to i64
  %t1925 = bitcast i64 %t1936 to double
  %t1937 = bitcast double %t1925 to i64
  %t1938 = alloca i64
  store i64 %t1937, i64* %t1938
  %t1939 = load i64, i64* %t1938
  %t1940 = inttoptr i64 %t1939 to i8*
  call i32 @puts(i8* %t1940)
  %t1942 = ptrtoint i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.slit_1941, i64 0, i64 0) to i64
  %t1943 = bitcast i64 %t1942 to double
  %t1945 = bitcast double %t1943 to i64
  %t1946 = alloca i64
  store i64 %t1945, i64* %t1946
  %t1947 = load i64, i64* %t1946
  %t1948 = inttoptr i64 %t1947 to i8*
  %t1949 = call i8* @__hulk_num_to_str(double -4.2e1)
  %t1950 = call i64 @strlen(i8* %t1948)
  %t1951 = call i64 @strlen(i8* %t1949)
  %t1952 = add i64 %t1950, %t1951
  %t1953 = add i64 %t1952, 2
  %t1954 = call i8* @malloc(i64 %t1953)
  call void @__hulk_gc_track(i8* %t1954)
  call i8* @strcpy(i8* %t1954, i8* %t1948)
  call i8* @strcat(i8* %t1954, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1954, i8* %t1949)
  %t1955 = ptrtoint i8* %t1954 to i64
  %t1944 = bitcast i64 %t1955 to double
  %t1956 = bitcast double %t1944 to i64
  %t1957 = alloca i64
  store i64 %t1956, i64* %t1957
  %t1958 = load i64, i64* %t1957
  %t1959 = inttoptr i64 %t1958 to i8*
  call i32 @puts(i8* %t1959)
  %t1961 = ptrtoint i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.slit_1960, i64 0, i64 0) to i64
  %t1962 = bitcast i64 %t1961 to double
  %t1963 = bitcast double %t1962 to i64
  %t1964 = alloca i64
  store i64 %t1963, i64* %t1964
  %t1965 = load i64, i64* %t1964
  %t1966 = inttoptr i64 %t1965 to i8*
  call i32 @puts(i8* %t1966)
  %t1968 = ptrtoint i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.slit_1967, i64 0, i64 0) to i64
  %t1969 = bitcast i64 %t1968 to double
  %t1970 = bitcast double %t1969 to i64
  %t1971 = alloca i64
  store i64 %t1970, i64* %t1971
  %t1972 = load i64, i64* %t1971
  %t1973 = inttoptr i64 %t1972 to i8*
  call i32 @puts(i8* %t1973)
  %t1975 = ptrtoint i8* getelementptr inbounds ([17 x i8], [17 x i8]* @.slit_1974, i64 0, i64 0) to i64
  %t1976 = bitcast i64 %t1975 to double
  %t1977 = bitcast double %t1976 to i64
  %t1978 = alloca i64
  store i64 %t1977, i64* %t1978
  %t1979 = load i64, i64* %t1978
  %t1980 = inttoptr i64 %t1979 to i8*
  call i32 @puts(i8* %t1980)
  %t1981 = fcmp one double 1.0, 0.0
  %t1982 = select i1 %t1981, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t1982)
  %t1984 = ptrtoint i8* getelementptr inbounds ([17 x i8], [17 x i8]* @.slit_1983, i64 0, i64 0) to i64
  %t1985 = bitcast i64 %t1984 to double
  %t1986 = bitcast double %t1985 to i64
  %t1987 = alloca i64
  store i64 %t1986, i64* %t1987
  %t1988 = load i64, i64* %t1987
  %t1989 = inttoptr i64 %t1988 to i8*
  call i32 @puts(i8* %t1989)
  %t1990 = fcmp one double 0.0, 0.0
  %t1991 = select i1 %t1990, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t1991)
  %t1992 = call double @separator()
  %t1994 = ptrtoint i8* getelementptr inbounds ([26 x i8], [26 x i8]* @.slit_1993, i64 0, i64 0) to i64
  %t1995 = bitcast i64 %t1994 to double
  %t1996 = bitcast double %t1995 to i64
  %t1997 = alloca i64
  store i64 %t1996, i64* %t1997
  %t1998 = load i64, i64* %t1997
  %t1999 = inttoptr i64 %t1998 to i8*
  call i32 @puts(i8* %t1999)
  %t2000 = call i8* @Doubler_new()
  %t2001 = ptrtoint i8* %t2000 to i64
  %t2002 = bitcast i64 %t2001 to double
  %t2003 = alloca double
  store double %t2002, double* %t2003
  %t2005 = ptrtoint i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.slit_2004, i64 0, i64 0) to i64
  %t2006 = bitcast i64 %t2005 to double
  %t2007 = load double, double* %t2003
  %t2008 = bitcast double %t2007 to i64
  %t2009 = alloca i64
  store i64 %t2008, i64* %t2009
  %t2010 = load i64, i64* %t2009
  %t2011 = inttoptr i64 %t2010 to i8*
  %t2012 = call double @Doubler_invoke(i8* %t2011, double 1.0e1)
  %t2014 = bitcast double %t2006 to i64
  %t2015 = alloca i64
  store i64 %t2014, i64* %t2015
  %t2016 = load i64, i64* %t2015
  %t2017 = inttoptr i64 %t2016 to i8*
  %t2018 = call i8* @__hulk_num_to_str(double %t2012)
  %t2019 = call i64 @strlen(i8* %t2017)
  %t2020 = call i64 @strlen(i8* %t2018)
  %t2021 = add i64 %t2019, %t2020
  %t2022 = add i64 %t2021, 2
  %t2023 = call i8* @malloc(i64 %t2022)
  call void @__hulk_gc_track(i8* %t2023)
  call i8* @strcpy(i8* %t2023, i8* %t2017)
  call i8* @strcat(i8* %t2023, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t2023, i8* %t2018)
  %t2024 = ptrtoint i8* %t2023 to i64
  %t2013 = bitcast i64 %t2024 to double
  %t2025 = bitcast double %t2013 to i64
  %t2026 = alloca i64
  store i64 %t2025, i64* %t2026
  %t2027 = load i64, i64* %t2026
  %t2028 = inttoptr i64 %t2027 to i8*
  call i32 @puts(i8* %t2028)
  %t2030 = ptrtoint i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.slit_2029, i64 0, i64 0) to i64
  %t2031 = bitcast i64 %t2030 to double
  %t2032 = load double, double* %t2003
  %t2033 = bitcast double %t2032 to i64
  %t2034 = alloca i64
  store i64 %t2033, i64* %t2034
  %t2035 = load i64, i64* %t2034
  %t2036 = inttoptr i64 %t2035 to i8*
  %t2037 = call double @Doubler_invoke(i8* %t2036, double 2.5e1)
  %t2039 = bitcast double %t2031 to i64
  %t2040 = alloca i64
  store i64 %t2039, i64* %t2040
  %t2041 = load i64, i64* %t2040
  %t2042 = inttoptr i64 %t2041 to i8*
  %t2043 = call i8* @__hulk_num_to_str(double %t2037)
  %t2044 = call i64 @strlen(i8* %t2042)
  %t2045 = call i64 @strlen(i8* %t2043)
  %t2046 = add i64 %t2044, %t2045
  %t2047 = add i64 %t2046, 2
  %t2048 = call i8* @malloc(i64 %t2047)
  call void @__hulk_gc_track(i8* %t2048)
  call i8* @strcpy(i8* %t2048, i8* %t2042)
  call i8* @strcat(i8* %t2048, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t2048, i8* %t2043)
  %t2049 = ptrtoint i8* %t2048 to i64
  %t2038 = bitcast i64 %t2049 to double
  %t2050 = bitcast double %t2038 to i64
  %t2051 = alloca i64
  store i64 %t2050, i64* %t2051
  %t2052 = load i64, i64* %t2051
  %t2053 = inttoptr i64 %t2052 to i8*
  call i32 @puts(i8* %t2053)
  %t2054 = call i8* @Adder_new(double 5.0e0)
  %t2055 = ptrtoint i8* %t2054 to i64
  %t2056 = bitcast i64 %t2055 to double
  %t2057 = alloca double
  store double %t2056, double* %t2057
  %t2059 = ptrtoint i8* getelementptr inbounds ([15 x i8], [15 x i8]* @.slit_2058, i64 0, i64 0) to i64
  %t2060 = bitcast i64 %t2059 to double
  %t2061 = load double, double* %t2057
  %t2062 = bitcast double %t2061 to i64
  %t2063 = alloca i64
  store i64 %t2062, i64* %t2063
  %t2064 = load i64, i64* %t2063
  %t2065 = inttoptr i64 %t2064 to i8*
  %t2066 = call double @Adder_invoke(i8* %t2065, double 1.0e1)
  %t2068 = bitcast double %t2060 to i64
  %t2069 = alloca i64
  store i64 %t2068, i64* %t2069
  %t2070 = load i64, i64* %t2069
  %t2071 = inttoptr i64 %t2070 to i8*
  %t2072 = call i8* @__hulk_num_to_str(double %t2066)
  %t2073 = call i64 @strlen(i8* %t2071)
  %t2074 = call i64 @strlen(i8* %t2072)
  %t2075 = add i64 %t2073, %t2074
  %t2076 = add i64 %t2075, 2
  %t2077 = call i8* @malloc(i64 %t2076)
  call void @__hulk_gc_track(i8* %t2077)
  call i8* @strcpy(i8* %t2077, i8* %t2071)
  call i8* @strcat(i8* %t2077, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t2077, i8* %t2072)
  %t2078 = ptrtoint i8* %t2077 to i64
  %t2067 = bitcast i64 %t2078 to double
  %t2079 = bitcast double %t2067 to i64
  %t2080 = alloca i64
  store i64 %t2079, i64* %t2080
  %t2081 = load i64, i64* %t2080
  %t2082 = inttoptr i64 %t2081 to i8*
  call i32 @puts(i8* %t2082)
  %t2084 = ptrtoint i8* getelementptr inbounds ([16 x i8], [16 x i8]* @.slit_2083, i64 0, i64 0) to i64
  %t2085 = bitcast i64 %t2084 to double
  %t2086 = load double, double* %t2057
  %t2087 = bitcast double %t2086 to i64
  %t2088 = alloca i64
  store i64 %t2087, i64* %t2088
  %t2089 = load i64, i64* %t2088
  %t2090 = inttoptr i64 %t2089 to i8*
  %t2091 = call double @Adder_invoke(i8* %t2090, double 1.0e2)
  %t2093 = bitcast double %t2085 to i64
  %t2094 = alloca i64
  store i64 %t2093, i64* %t2094
  %t2095 = load i64, i64* %t2094
  %t2096 = inttoptr i64 %t2095 to i8*
  %t2097 = call i8* @__hulk_num_to_str(double %t2091)
  %t2098 = call i64 @strlen(i8* %t2096)
  %t2099 = call i64 @strlen(i8* %t2097)
  %t2100 = add i64 %t2098, %t2099
  %t2101 = add i64 %t2100, 2
  %t2102 = call i8* @malloc(i64 %t2101)
  call void @__hulk_gc_track(i8* %t2102)
  call i8* @strcpy(i8* %t2102, i8* %t2096)
  call i8* @strcat(i8* %t2102, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t2102, i8* %t2097)
  %t2103 = ptrtoint i8* %t2102 to i64
  %t2092 = bitcast i64 %t2103 to double
  %t2104 = bitcast double %t2092 to i64
  %t2105 = alloca i64
  store i64 %t2104, i64* %t2105
  %t2106 = load i64, i64* %t2105
  %t2107 = inttoptr i64 %t2106 to i8*
  call i32 @puts(i8* %t2107)
  %t2109 = ptrtoint i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.slit_2108, i64 0, i64 0) to i64
  %t2110 = bitcast i64 %t2109 to double
  %t2111 = call i8* @Greeter_new(double %t2110)
  %t2112 = ptrtoint i8* %t2111 to i64
  %t2113 = bitcast i64 %t2112 to double
  %t2114 = alloca double
  store double %t2113, double* %t2114
  %t2115 = load double, double* %t2114
  %t2116 = bitcast double %t2115 to i64
  %t2117 = alloca i64
  store i64 %t2116, i64* %t2117
  %t2118 = load i64, i64* %t2117
  %t2119 = inttoptr i64 %t2118 to i8*
  %t2121 = ptrtoint i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.slit_2120, i64 0, i64 0) to i64
  %t2122 = bitcast i64 %t2121 to double
  %t2123 = call double @Greeter_invoke(i8* %t2119, double %t2122)
  %t2124 = bitcast double %t2123 to i64
  %t2125 = alloca i64
  store i64 %t2124, i64* %t2125
  %t2126 = load i64, i64* %t2125
  %t2127 = inttoptr i64 %t2126 to i8*
  call i32 @puts(i8* %t2127)
  %t2128 = load double, double* %t2114
  %t2129 = bitcast double %t2128 to i64
  %t2130 = alloca i64
  store i64 %t2129, i64* %t2130
  %t2131 = load i64, i64* %t2130
  %t2132 = inttoptr i64 %t2131 to i8*
  %t2134 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_2133, i64 0, i64 0) to i64
  %t2135 = bitcast i64 %t2134 to double
  %t2136 = call double @Greeter_invoke(i8* %t2132, double %t2135)
  %t2137 = bitcast double %t2136 to i64
  %t2138 = alloca i64
  store i64 %t2137, i64* %t2138
  %t2139 = load i64, i64* %t2138
  %t2140 = inttoptr i64 %t2139 to i8*
  call i32 @puts(i8* %t2140)
  %t2141 = call double @separator()
  %t2143 = ptrtoint i8* getelementptr inbounds ([25 x i8], [25 x i8]* @.slit_2142, i64 0, i64 0) to i64
  %t2144 = bitcast i64 %t2143 to double
  %t2145 = bitcast double %t2144 to i64
  %t2146 = alloca i64
  store i64 %t2145, i64* %t2146
  %t2147 = load i64, i64* %t2146
  %t2148 = inttoptr i64 %t2147 to i8*
  call i32 @puts(i8* %t2148)
  %t2150 = ptrtoint i8* getelementptr inbounds ([21 x i8], [21 x i8]* @.slit_2149, i64 0, i64 0) to i64
  %t2151 = bitcast i64 %t2150 to double
  %t2152 = bitcast double %t2151 to i64
  %t2153 = alloca i64
  store i64 %t2152, i64* %t2153
  %t2154 = load i64, i64* %t2153
  %t2155 = inttoptr i64 %t2154 to i8*
  call i32 @puts(i8* %t2155)
  %t2156 = load double, double* %t640
  %t2157 = bitcast double %t2156 to i64
  %t2158 = alloca i64
  store i64 %t2157, i64* %t2158
  %t2159 = load i64, i64* %t2158
  %t2160 = inttoptr i64 %t2159 to i8*
  %t2161 = call double @Dog_describe(i8* %t2160)
  %t2162 = bitcast double %t2161 to i64
  %t2163 = alloca i64
  store i64 %t2162, i64* %t2163
  %t2164 = load i64, i64* %t2163
  %t2165 = inttoptr i64 %t2164 to i8*
  call i32 @puts(i8* %t2165)
  %t2166 = load double, double* %t657
  %t2167 = bitcast double %t2166 to i64
  %t2168 = alloca i64
  store i64 %t2167, i64* %t2168
  %t2169 = load i64, i64* %t2168
  %t2170 = inttoptr i64 %t2169 to i8*
  %t2171 = call double @Cat_describe(i8* %t2170)
  %t2172 = bitcast double %t2171 to i64
  %t2173 = alloca i64
  store i64 %t2172, i64* %t2173
  %t2174 = load i64, i64* %t2173
  %t2175 = inttoptr i64 %t2174 to i8*
  call i32 @puts(i8* %t2175)
  %t2176 = load double, double* %t671
  %t2177 = bitcast double %t2176 to i64
  %t2178 = alloca i64
  store i64 %t2177, i64* %t2178
  %t2179 = load i64, i64* %t2178
  %t2180 = inttoptr i64 %t2179 to i8*
  %t2181 = call double @Bird_describe(i8* %t2180)
  %t2182 = bitcast double %t2181 to i64
  %t2183 = alloca i64
  store i64 %t2182, i64* %t2183
  %t2184 = load i64, i64* %t2183
  %t2185 = inttoptr i64 %t2184 to i8*
  call i32 @puts(i8* %t2185)
  %t2186 = call double @separator()
  %t2188 = ptrtoint i8* getelementptr inbounds ([27 x i8], [27 x i8]* @.slit_2187, i64 0, i64 0) to i64
  %t2189 = bitcast i64 %t2188 to double
  %t2190 = bitcast double %t2189 to i64
  %t2191 = alloca i64
  store i64 %t2190, i64* %t2191
  %t2192 = load i64, i64* %t2191
  %t2193 = inttoptr i64 %t2192 to i8*
  call i32 @puts(i8* %t2193)
  call void @__hulk_gc_sweep()
  ret i32 0
}

define double @__lambda_1139(double* %__env, double %x) {
entry:
  %t1140 = alloca double
  store double %x, double* %t1140
  %t1141 = load double, double* %t1140
  %t1142 = load double, double* %t1140
  %t1143 = fmul double %t1141, %t1142
  ret double %t1143
}

define double @__lambda_1189(double* %__env, double %x) {
entry:
  %t1190 = alloca double
  store double %x, double* %t1190
  %t1191 = load double, double* %t1190
  %t1192 = fmul double %t1191, 3.0e0
  ret double %t1192
}

define double @__lambda_1273(double* %__env, double %name) {
entry:
  %t1274 = alloca double
  store double %name, double* %t1274
  %t1276 = ptrtoint i8* getelementptr inbounds ([8 x i8], [8 x i8]* @.slit_1275, i64 0, i64 0) to i64
  %t1277 = bitcast i64 %t1276 to double
  %t1278 = load double, double* %t1274
  %t1280 = bitcast double %t1277 to i64
  %t1281 = alloca i64
  store i64 %t1280, i64* %t1281
  %t1282 = load i64, i64* %t1281
  %t1283 = inttoptr i64 %t1282 to i8*
  %t1284 = bitcast double %t1278 to i64
  %t1285 = alloca i64
  store i64 %t1284, i64* %t1285
  %t1286 = load i64, i64* %t1285
  %t1287 = inttoptr i64 %t1286 to i8*
  %t1288 = call i64 @strlen(i8* %t1283)
  %t1289 = call i64 @strlen(i8* %t1287)
  %t1290 = add i64 %t1288, %t1289
  %t1291 = add i64 %t1290, 1
  %t1292 = call i8* @malloc(i64 %t1291)
  call void @__hulk_gc_track(i8* %t1292)
  call i8* @strcpy(i8* %t1292, i8* %t1283)
  call i8* @strcat(i8* %t1292, i8* %t1287)
  %t1293 = ptrtoint i8* %t1292 to i64
  %t1279 = bitcast i64 %t1293 to double
  %t1295 = ptrtoint i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.slit_1294, i64 0, i64 0) to i64
  %t1296 = bitcast i64 %t1295 to double
  %t1298 = bitcast double %t1279 to i64
  %t1299 = alloca i64
  store i64 %t1298, i64* %t1299
  %t1300 = load i64, i64* %t1299
  %t1301 = inttoptr i64 %t1300 to i8*
  %t1302 = bitcast double %t1296 to i64
  %t1303 = alloca i64
  store i64 %t1302, i64* %t1303
  %t1304 = load i64, i64* %t1303
  %t1305 = inttoptr i64 %t1304 to i8*
  %t1306 = call i64 @strlen(i8* %t1301)
  %t1307 = call i64 @strlen(i8* %t1305)
  %t1308 = add i64 %t1306, %t1307
  %t1309 = add i64 %t1308, 1
  %t1310 = call i8* @malloc(i64 %t1309)
  call void @__hulk_gc_track(i8* %t1310)
  call i8* @strcpy(i8* %t1310, i8* %t1301)
  call i8* @strcat(i8* %t1310, i8* %t1305)
  %t1311 = ptrtoint i8* %t1310 to i64
  %t1297 = bitcast i64 %t1311 to double
  ret double %t1297
}

