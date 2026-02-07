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

%T.Animal = type { i64, double, double }
@.slit_9 = private unnamed_addr constant [6 x i8] c"I am \00"
@.slit_36 = private unnamed_addr constant [7 x i8] c" with \00"
@.slit_75 = private unnamed_addr constant [6 x i8] c" legs\00"
%T.Mammal = type { i64, double, double, double }
@.slit_136 = private unnamed_addr constant [13 x i8] c"warm-blooded\00"
@.slit_139 = private unnamed_addr constant [13 x i8] c"cold-blooded\00"
%T.Dog = type { i64, double, double, double, double }
@.slit_171 = private unnamed_addr constant [13 x i8] c" says: Woof!\00"
@.slit_198 = private unnamed_addr constant [6 x i8] c" the \00"
%T.Bird = type { i64, double, double, double }
@.slit_279 = private unnamed_addr constant [10 x i8] c" can fly!\00"
@.slit_306 = private unnamed_addr constant [12 x i8] c" cannot fly\00"
%T.Cat = type { i64, double, double, double, double }
@.slit_353 = private unnamed_addr constant [13 x i8] c" says: Meow!\00"
@.slit_394 = private unnamed_addr constant [14 x i8] c" (indoor cat)\00"
@.slit_421 = private unnamed_addr constant [15 x i8] c" (outdoor cat)\00"
@.slit_440 = private unnamed_addr constant [97 x i8] c"────────────────────────────────\00"
@.slit_498 = private unnamed_addr constant [1 x i8] c"\00"
@.slit_531 = private unnamed_addr constant [30 x i8] c"=== ZOO MANAGEMENT SYSTEM ===\00"
@.slit_539 = private unnamed_addr constant [4 x i8] c"Rex\00"
@.slit_542 = private unnamed_addr constant [9 x i8] c"Labrador\00"
@.slit_549 = private unnamed_addr constant [5 x i8] c"Milo\00"
@.slit_552 = private unnamed_addr constant [7 x i8] c"Beagle\00"
@.slit_559 = private unnamed_addr constant [5 x i8] c"Luna\00"
@.slit_566 = private unnamed_addr constant [6 x i8] c"Simba\00"
@.slit_573 = private unnamed_addr constant [7 x i8] c"Tweety\00"
@.slit_580 = private unnamed_addr constant [5 x i8] c"Kiwi\00"
@.slit_587 = private unnamed_addr constant [25 x i8] c"--- Animal Greetings ---\00"
@.slit_625 = private unnamed_addr constant [20 x i8] c"--- Dog Actions ---\00"
@.slit_673 = private unnamed_addr constant [20 x i8] c"--- Cat Actions ---\00"
@.slit_721 = private unnamed_addr constant [18 x i8] c"--- Bird Info ---\00"
@.slit_749 = private unnamed_addr constant [26 x i8] c"--- Mammal Properties ---\00"
@.slit_777 = private unnamed_addr constant [25 x i8] c"--- Type Checks (is) ---\00"
@.slit_784 = private unnamed_addr constant [15 x i8] c"Rex is Animal?\00"
@.slit_816 = private unnamed_addr constant [15 x i8] c"Rex is Mammal?\00"
@.slit_842 = private unnamed_addr constant [12 x i8] c"Rex is Dog?\00"
@.slit_862 = private unnamed_addr constant [18 x i8] c"Tweety is Mammal?\00"
@.slit_888 = private unnamed_addr constant [18 x i8] c"Tweety is Animal?\00"
@.slit_921 = private unnamed_addr constant [29 x i8] c"--- Zoo Census (Vectors) ---\00"
@.slit_963 = private unnamed_addr constant [19 x i8] c"Total legs in zoo:\00"
@.slit_1011 = private unnamed_addr constant [20 x i8] c"Doubled leg counts:\00"
@.slit_1040 = private unnamed_addr constant [31 x i8] c"--- Lambda & Closure Tests ---\00"
@.slit_1060 = private unnamed_addr constant [12 x i8] c"5 squared =\00"
@.slit_1109 = private unnamed_addr constant [17 x i8] c"10 scaled by 3 =\00"
@.slit_1145 = private unnamed_addr constant [16 x i8] c"7 scaled by 3 =\00"
@.slit_1183 = private unnamed_addr constant [8 x i8] c"Hello, \00"
@.slit_1202 = private unnamed_addr constant [2 x i8] c"!\00"
@.slit_1228 = private unnamed_addr constant [6 x i8] c"World\00"
@.slit_1252 = private unnamed_addr constant [5 x i8] c"HULK\00"
@.slit_1277 = private unnamed_addr constant [23 x i8] c"--- Math Functions ---\00"
@.slit_1284 = private unnamed_addr constant [5 x i8] c"PI =\00"
@.slit_1303 = private unnamed_addr constant [5 x i8] c"E  =\00"
@.slit_1322 = private unnamed_addr constant [12 x i8] c"sqrt(144) =\00"
@.slit_1341 = private unnamed_addr constant [9 x i8] c"sin(0) =\00"
@.slit_1360 = private unnamed_addr constant [9 x i8] c"cos(0) =\00"
@.slit_1379 = private unnamed_addr constant [9 x i8] c"exp(1) =\00"
@.slit_1398 = private unnamed_addr constant [16 x i8] c"log(10, 1000) =\00"
@.slit_1426 = private unnamed_addr constant [14 x i8] c"Random [0,1):\00"
@.slit_1435 = private unnamed_addr constant [31 x i8] c"--- Fibonacci (while loop) ---\00"
@.slit_1463 = private unnamed_addr constant [26 x i8] c"--- Variable Mutation ---\00"
@.slit_1479 = private unnamed_addr constant [10 x i8] c"counter =\00"
@.slit_1504 = private unnamed_addr constant [16 x i8] c"Final counter =\00"
@.slit_1525 = private unnamed_addr constant [26 x i8] c"--- String Operations ---\00"
@.slit_1532 = private unnamed_addr constant [5 x i8] c"HULK\00"
@.slit_1536 = private unnamed_addr constant [9 x i8] c"Compiler\00"
@.slit_1540 = private unnamed_addr constant [4 x i8] c"2.0\00"
@.slit_1544 = private unnamed_addr constant [19 x i8] c"HULK Compiler v2.0\00"
@.slit_1551 = private unnamed_addr constant [18 x i8] c"HULK Compiler 2.0\00"
@.slit_1558 = private unnamed_addr constant [2 x i8] c"*\00"
@.slit_1564 = private unnamed_addr constant [2 x i8] c" \00"
@.slit_1582 = private unnamed_addr constant [5 x i8] c"HULK\00"
@.slit_1600 = private unnamed_addr constant [2 x i8] c" \00"
@.slit_1639 = private unnamed_addr constant [26 x i8] c"--- Utility Functions ---\00"
@.slit_1646 = private unnamed_addr constant [11 x i8] c"abs(-42) =\00"
@.slit_1666 = private unnamed_addr constant [14 x i8] c"max(10, 20) =\00"
@.slit_1686 = private unnamed_addr constant [14 x i8] c"min(10, 20) =\00"
@.slit_1706 = private unnamed_addr constant [21 x i8] c"clamp(150, 0, 100) =\00"
@.slit_1726 = private unnamed_addr constant [20 x i8] c"clamp(-5, 0, 100) =\00"
@.slit_1747 = private unnamed_addr constant [22 x i8] c"--- Is Quadruped? ---\00"
@.slit_1754 = private unnamed_addr constant [14 x i8] c"Rex (4 legs):\00"
@.slit_1769 = private unnamed_addr constant [17 x i8] c"Tweety (2 legs):\00"
@.slit_1785 = private unnamed_addr constant [27 x i8] c"=== ALL TESTS COMPLETE ===\00"

define i8* @Animal_new(double %name, double %legs) {
entry:
  %t0 = call i8* @malloc(i64 24)
  call void @__hulk_gc_track(i8* %t0)
  %t1 = bitcast i8* %t0 to %T.Animal*
  %t2 = getelementptr inbounds %T.Animal, %T.Animal* %t1, i32 0, i32 0
  store i64 1, i64* %t2
  %t3 = alloca double
  store double %name, double* %t3
  %t4 = alloca double
  store double %legs, double* %t4
  %t5 = load double, double* %t3
  %t6 = getelementptr inbounds %T.Animal, %T.Animal* %t1, i32 0, i32 1
  store double %t5, double* %t6
  %t7 = load double, double* %t4
  %t8 = getelementptr inbounds %T.Animal, %T.Animal* %t1, i32 0, i32 2
  store double %t7, double* %t8
  ret i8* %t0
}

define double @Animal_greet(i8* %self) {
entry:
  %t10 = ptrtoint i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.slit_9, i64 0, i64 0) to i64
  %t11 = bitcast i64 %t10 to double
  %t12 = ptrtoint i8* %self to i64
  %t13 = bitcast i64 %t12 to double
  %t14 = bitcast double %t13 to i64
  %t15 = alloca i64
  store i64 %t14, i64* %t15
  %t16 = load i64, i64* %t15
  %t17 = inttoptr i64 %t16 to i8*
  %t18 = bitcast i8* %t17 to %T.Animal*
  %t19 = getelementptr inbounds %T.Animal, %T.Animal* %t18, i32 0, i32 1
  %t20 = load double, double* %t19
  %t22 = bitcast double %t11 to i64
  %t23 = alloca i64
  store i64 %t22, i64* %t23
  %t24 = load i64, i64* %t23
  %t25 = inttoptr i64 %t24 to i8*
  %t26 = bitcast double %t20 to i64
  %t27 = alloca i64
  store i64 %t26, i64* %t27
  %t28 = load i64, i64* %t27
  %t29 = inttoptr i64 %t28 to i8*
  %t30 = call i64 @strlen(i8* %t25)
  %t31 = call i64 @strlen(i8* %t29)
  %t32 = add i64 %t30, %t31
  %t33 = add i64 %t32, 1
  %t34 = call i8* @malloc(i64 %t33)
  call void @__hulk_gc_track(i8* %t34)
  call i8* @strcpy(i8* %t34, i8* %t25)
  call i8* @strcat(i8* %t34, i8* %t29)
  %t35 = ptrtoint i8* %t34 to i64
  %t21 = bitcast i64 %t35 to double
  %t37 = ptrtoint i8* getelementptr inbounds ([7 x i8], [7 x i8]* @.slit_36, i64 0, i64 0) to i64
  %t38 = bitcast i64 %t37 to double
  %t40 = bitcast double %t21 to i64
  %t41 = alloca i64
  store i64 %t40, i64* %t41
  %t42 = load i64, i64* %t41
  %t43 = inttoptr i64 %t42 to i8*
  %t44 = bitcast double %t38 to i64
  %t45 = alloca i64
  store i64 %t44, i64* %t45
  %t46 = load i64, i64* %t45
  %t47 = inttoptr i64 %t46 to i8*
  %t48 = call i64 @strlen(i8* %t43)
  %t49 = call i64 @strlen(i8* %t47)
  %t50 = add i64 %t48, %t49
  %t51 = add i64 %t50, 1
  %t52 = call i8* @malloc(i64 %t51)
  call void @__hulk_gc_track(i8* %t52)
  call i8* @strcpy(i8* %t52, i8* %t43)
  call i8* @strcat(i8* %t52, i8* %t47)
  %t53 = ptrtoint i8* %t52 to i64
  %t39 = bitcast i64 %t53 to double
  %t54 = ptrtoint i8* %self to i64
  %t55 = bitcast i64 %t54 to double
  %t56 = bitcast double %t55 to i64
  %t57 = alloca i64
  store i64 %t56, i64* %t57
  %t58 = load i64, i64* %t57
  %t59 = inttoptr i64 %t58 to i8*
  %t60 = bitcast i8* %t59 to %T.Animal*
  %t61 = getelementptr inbounds %T.Animal, %T.Animal* %t60, i32 0, i32 2
  %t62 = load double, double* %t61
  %t64 = bitcast double %t39 to i64
  %t65 = alloca i64
  store i64 %t64, i64* %t65
  %t66 = load i64, i64* %t65
  %t67 = inttoptr i64 %t66 to i8*
  %t68 = call i8* @__hulk_num_to_str(double %t62)
  %t69 = call i64 @strlen(i8* %t67)
  %t70 = call i64 @strlen(i8* %t68)
  %t71 = add i64 %t69, %t70
  %t72 = add i64 %t71, 2
  %t73 = call i8* @malloc(i64 %t72)
  call void @__hulk_gc_track(i8* %t73)
  call i8* @strcpy(i8* %t73, i8* %t67)
  call i8* @strcat(i8* %t73, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t73, i8* %t68)
  %t74 = ptrtoint i8* %t73 to i64
  %t63 = bitcast i64 %t74 to double
  %t76 = ptrtoint i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.slit_75, i64 0, i64 0) to i64
  %t77 = bitcast i64 %t76 to double
  %t79 = bitcast double %t63 to i64
  %t80 = alloca i64
  store i64 %t79, i64* %t80
  %t81 = load i64, i64* %t80
  %t82 = inttoptr i64 %t81 to i8*
  %t83 = bitcast double %t77 to i64
  %t84 = alloca i64
  store i64 %t83, i64* %t84
  %t85 = load i64, i64* %t84
  %t86 = inttoptr i64 %t85 to i8*
  %t87 = call i64 @strlen(i8* %t82)
  %t88 = call i64 @strlen(i8* %t86)
  %t89 = add i64 %t87, %t88
  %t90 = add i64 %t89, 2
  %t91 = call i8* @malloc(i64 %t90)
  call void @__hulk_gc_track(i8* %t91)
  call i8* @strcpy(i8* %t91, i8* %t82)
  call i8* @strcat(i8* %t91, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t91, i8* %t86)
  %t92 = ptrtoint i8* %t91 to i64
  %t78 = bitcast i64 %t92 to double
  ret double %t78
}

define double @Animal_is_quadruped(i8* %self) {
entry:
  %t93 = ptrtoint i8* %self to i64
  %t94 = bitcast i64 %t93 to double
  %t95 = bitcast double %t94 to i64
  %t96 = alloca i64
  store i64 %t95, i64* %t96
  %t97 = load i64, i64* %t96
  %t98 = inttoptr i64 %t97 to i8*
  %t99 = bitcast i8* %t98 to %T.Animal*
  %t100 = getelementptr inbounds %T.Animal, %T.Animal* %t99, i32 0, i32 2
  %t101 = load double, double* %t100
  %t103 = fcmp oeq double %t101, 4.0e0
  %t102 = uitofp i1 %t103 to double
  ret double %t102
}

define i8* @Mammal_new(double %name, double %legs, double %warm) {
entry:
  %t104 = call i8* @malloc(i64 32)
  call void @__hulk_gc_track(i8* %t104)
  %t105 = bitcast i8* %t104 to %T.Mammal*
  %t106 = getelementptr inbounds %T.Mammal, %T.Mammal* %t105, i32 0, i32 0
  store i64 2, i64* %t106
  %t107 = alloca double
  store double %name, double* %t107
  %t108 = alloca double
  store double %legs, double* %t108
  %t109 = alloca double
  store double %warm, double* %t109
  %t110 = load double, double* %t107
  %t111 = load double, double* %t108
  %t112 = call i8* @Animal_new(double %t110, double %t111)
  %t113 = bitcast i8* %t112 to %T.Animal*
  %t114 = getelementptr inbounds %T.Animal, %T.Animal* %t113, i32 0, i32 1
  %t115 = load double, double* %t114
  %t116 = getelementptr inbounds %T.Mammal, %T.Mammal* %t105, i32 0, i32 1
  store double %t115, double* %t116
  %t117 = getelementptr inbounds %T.Animal, %T.Animal* %t113, i32 0, i32 2
  %t118 = load double, double* %t117
  %t119 = getelementptr inbounds %T.Mammal, %T.Mammal* %t105, i32 0, i32 2
  store double %t118, double* %t119
  %t120 = load double, double* %t109
  %t121 = getelementptr inbounds %T.Mammal, %T.Mammal* %t105, i32 0, i32 3
  store double %t120, double* %t121
  ret i8* %t104
}

define double @Mammal_body_temp(i8* %self) {
entry:
  %t122 = ptrtoint i8* %self to i64
  %t123 = bitcast i64 %t122 to double
  %t124 = bitcast double %t123 to i64
  %t125 = alloca i64
  store i64 %t124, i64* %t125
  %t126 = load i64, i64* %t125
  %t127 = inttoptr i64 %t126 to i8*
  %t128 = bitcast i8* %t127 to %T.Mammal*
  %t129 = getelementptr inbounds %T.Mammal, %T.Mammal* %t128, i32 0, i32 3
  %t130 = load double, double* %t129
  %t131 = fcmp one double %t130, 0.0
  %t132 = alloca double
  br i1 %t131, label %then_133, label %else_134
then_133:
  %t137 = ptrtoint i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.slit_136, i64 0, i64 0) to i64
  %t138 = bitcast i64 %t137 to double
  store double %t138, double* %t132
  br label %merge_135
else_134:
  %t140 = ptrtoint i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.slit_139, i64 0, i64 0) to i64
  %t141 = bitcast i64 %t140 to double
  store double %t141, double* %t132
  br label %merge_135
merge_135:
  %t142 = load double, double* %t132
  ret double %t142
}

define i8* @Dog_new(double %name, double %breed) {
entry:
  %t143 = call i8* @malloc(i64 40)
  call void @__hulk_gc_track(i8* %t143)
  %t144 = bitcast i8* %t143 to %T.Dog*
  %t145 = getelementptr inbounds %T.Dog, %T.Dog* %t144, i32 0, i32 0
  store i64 3, i64* %t145
  %t146 = alloca double
  store double %name, double* %t146
  %t147 = alloca double
  store double %breed, double* %t147
  %t148 = load double, double* %t146
  %t149 = call i8* @Mammal_new(double %t148, double 4.0e0, double 1.0)
  %t150 = bitcast i8* %t149 to %T.Mammal*
  %t151 = getelementptr inbounds %T.Mammal, %T.Mammal* %t150, i32 0, i32 1
  %t152 = load double, double* %t151
  %t153 = getelementptr inbounds %T.Dog, %T.Dog* %t144, i32 0, i32 1
  store double %t152, double* %t153
  %t154 = getelementptr inbounds %T.Mammal, %T.Mammal* %t150, i32 0, i32 2
  %t155 = load double, double* %t154
  %t156 = getelementptr inbounds %T.Dog, %T.Dog* %t144, i32 0, i32 2
  store double %t155, double* %t156
  %t157 = getelementptr inbounds %T.Mammal, %T.Mammal* %t150, i32 0, i32 3
  %t158 = load double, double* %t157
  %t159 = getelementptr inbounds %T.Dog, %T.Dog* %t144, i32 0, i32 3
  store double %t158, double* %t159
  %t160 = load double, double* %t147
  %t161 = getelementptr inbounds %T.Dog, %T.Dog* %t144, i32 0, i32 4
  store double %t160, double* %t161
  ret i8* %t143
}

define double @Dog_bark(i8* %self) {
entry:
  %t162 = ptrtoint i8* %self to i64
  %t163 = bitcast i64 %t162 to double
  %t164 = bitcast double %t163 to i64
  %t165 = alloca i64
  store i64 %t164, i64* %t165
  %t166 = load i64, i64* %t165
  %t167 = inttoptr i64 %t166 to i8*
  %t168 = bitcast i8* %t167 to %T.Dog*
  %t169 = getelementptr inbounds %T.Dog, %T.Dog* %t168, i32 0, i32 1
  %t170 = load double, double* %t169
  %t172 = ptrtoint i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.slit_171, i64 0, i64 0) to i64
  %t173 = bitcast i64 %t172 to double
  %t175 = bitcast double %t170 to i64
  %t176 = alloca i64
  store i64 %t175, i64* %t176
  %t177 = load i64, i64* %t176
  %t178 = inttoptr i64 %t177 to i8*
  %t179 = bitcast double %t173 to i64
  %t180 = alloca i64
  store i64 %t179, i64* %t180
  %t181 = load i64, i64* %t180
  %t182 = inttoptr i64 %t181 to i8*
  %t183 = call i64 @strlen(i8* %t178)
  %t184 = call i64 @strlen(i8* %t182)
  %t185 = add i64 %t183, %t184
  %t186 = add i64 %t185, 1
  %t187 = call i8* @malloc(i64 %t186)
  call void @__hulk_gc_track(i8* %t187)
  call i8* @strcpy(i8* %t187, i8* %t178)
  call i8* @strcat(i8* %t187, i8* %t182)
  %t188 = ptrtoint i8* %t187 to i64
  %t174 = bitcast i64 %t188 to double
  ret double %t174
}

define double @Dog_describe(i8* %self) {
entry:
  %t189 = ptrtoint i8* %self to i64
  %t190 = bitcast i64 %t189 to double
  %t191 = bitcast double %t190 to i64
  %t192 = alloca i64
  store i64 %t191, i64* %t192
  %t193 = load i64, i64* %t192
  %t194 = inttoptr i64 %t193 to i8*
  %t195 = bitcast i8* %t194 to %T.Dog*
  %t196 = getelementptr inbounds %T.Dog, %T.Dog* %t195, i32 0, i32 1
  %t197 = load double, double* %t196
  %t199 = ptrtoint i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.slit_198, i64 0, i64 0) to i64
  %t200 = bitcast i64 %t199 to double
  %t202 = bitcast double %t197 to i64
  %t203 = alloca i64
  store i64 %t202, i64* %t203
  %t204 = load i64, i64* %t203
  %t205 = inttoptr i64 %t204 to i8*
  %t206 = bitcast double %t200 to i64
  %t207 = alloca i64
  store i64 %t206, i64* %t207
  %t208 = load i64, i64* %t207
  %t209 = inttoptr i64 %t208 to i8*
  %t210 = call i64 @strlen(i8* %t205)
  %t211 = call i64 @strlen(i8* %t209)
  %t212 = add i64 %t210, %t211
  %t213 = add i64 %t212, 1
  %t214 = call i8* @malloc(i64 %t213)
  call void @__hulk_gc_track(i8* %t214)
  call i8* @strcpy(i8* %t214, i8* %t205)
  call i8* @strcat(i8* %t214, i8* %t209)
  %t215 = ptrtoint i8* %t214 to i64
  %t201 = bitcast i64 %t215 to double
  %t216 = ptrtoint i8* %self to i64
  %t217 = bitcast i64 %t216 to double
  %t218 = bitcast double %t217 to i64
  %t219 = alloca i64
  store i64 %t218, i64* %t219
  %t220 = load i64, i64* %t219
  %t221 = inttoptr i64 %t220 to i8*
  %t222 = bitcast i8* %t221 to %T.Dog*
  %t223 = getelementptr inbounds %T.Dog, %T.Dog* %t222, i32 0, i32 4
  %t224 = load double, double* %t223
  %t226 = bitcast double %t201 to i64
  %t227 = alloca i64
  store i64 %t226, i64* %t227
  %t228 = load i64, i64* %t227
  %t229 = inttoptr i64 %t228 to i8*
  %t230 = bitcast double %t224 to i64
  %t231 = alloca i64
  store i64 %t230, i64* %t231
  %t232 = load i64, i64* %t231
  %t233 = inttoptr i64 %t232 to i8*
  %t234 = call i64 @strlen(i8* %t229)
  %t235 = call i64 @strlen(i8* %t233)
  %t236 = add i64 %t234, %t235
  %t237 = add i64 %t236, 1
  %t238 = call i8* @malloc(i64 %t237)
  call void @__hulk_gc_track(i8* %t238)
  call i8* @strcpy(i8* %t238, i8* %t229)
  call i8* @strcat(i8* %t238, i8* %t233)
  %t239 = ptrtoint i8* %t238 to i64
  %t225 = bitcast i64 %t239 to double
  ret double %t225
}

define i8* @Bird_new(double %name, double %can_fly) {
entry:
  %t240 = call i8* @malloc(i64 32)
  call void @__hulk_gc_track(i8* %t240)
  %t241 = bitcast i8* %t240 to %T.Bird*
  %t242 = getelementptr inbounds %T.Bird, %T.Bird* %t241, i32 0, i32 0
  store i64 4, i64* %t242
  %t243 = alloca double
  store double %name, double* %t243
  %t244 = alloca double
  store double %can_fly, double* %t244
  %t245 = load double, double* %t243
  %t246 = call i8* @Animal_new(double %t245, double 2.0e0)
  %t247 = bitcast i8* %t246 to %T.Animal*
  %t248 = getelementptr inbounds %T.Animal, %T.Animal* %t247, i32 0, i32 1
  %t249 = load double, double* %t248
  %t250 = getelementptr inbounds %T.Bird, %T.Bird* %t241, i32 0, i32 1
  store double %t249, double* %t250
  %t251 = getelementptr inbounds %T.Animal, %T.Animal* %t247, i32 0, i32 2
  %t252 = load double, double* %t251
  %t253 = getelementptr inbounds %T.Bird, %T.Bird* %t241, i32 0, i32 2
  store double %t252, double* %t253
  %t254 = load double, double* %t244
  %t255 = getelementptr inbounds %T.Bird, %T.Bird* %t241, i32 0, i32 3
  store double %t254, double* %t255
  ret i8* %t240
}

define double @Bird_describe(i8* %self) {
entry:
  %t256 = ptrtoint i8* %self to i64
  %t257 = bitcast i64 %t256 to double
  %t258 = bitcast double %t257 to i64
  %t259 = alloca i64
  store i64 %t258, i64* %t259
  %t260 = load i64, i64* %t259
  %t261 = inttoptr i64 %t260 to i8*
  %t262 = bitcast i8* %t261 to %T.Bird*
  %t263 = getelementptr inbounds %T.Bird, %T.Bird* %t262, i32 0, i32 3
  %t264 = load double, double* %t263
  %t265 = fcmp one double %t264, 0.0
  %t266 = alloca double
  br i1 %t265, label %then_267, label %else_268
then_267:
  %t270 = ptrtoint i8* %self to i64
  %t271 = bitcast i64 %t270 to double
  %t272 = bitcast double %t271 to i64
  %t273 = alloca i64
  store i64 %t272, i64* %t273
  %t274 = load i64, i64* %t273
  %t275 = inttoptr i64 %t274 to i8*
  %t276 = bitcast i8* %t275 to %T.Bird*
  %t277 = getelementptr inbounds %T.Bird, %T.Bird* %t276, i32 0, i32 1
  %t278 = load double, double* %t277
  %t280 = ptrtoint i8* getelementptr inbounds ([10 x i8], [10 x i8]* @.slit_279, i64 0, i64 0) to i64
  %t281 = bitcast i64 %t280 to double
  %t283 = bitcast double %t278 to i64
  %t284 = alloca i64
  store i64 %t283, i64* %t284
  %t285 = load i64, i64* %t284
  %t286 = inttoptr i64 %t285 to i8*
  %t287 = bitcast double %t281 to i64
  %t288 = alloca i64
  store i64 %t287, i64* %t288
  %t289 = load i64, i64* %t288
  %t290 = inttoptr i64 %t289 to i8*
  %t291 = call i64 @strlen(i8* %t286)
  %t292 = call i64 @strlen(i8* %t290)
  %t293 = add i64 %t291, %t292
  %t294 = add i64 %t293, 1
  %t295 = call i8* @malloc(i64 %t294)
  call void @__hulk_gc_track(i8* %t295)
  call i8* @strcpy(i8* %t295, i8* %t286)
  call i8* @strcat(i8* %t295, i8* %t290)
  %t296 = ptrtoint i8* %t295 to i64
  %t282 = bitcast i64 %t296 to double
  store double %t282, double* %t266
  br label %merge_269
else_268:
  %t297 = ptrtoint i8* %self to i64
  %t298 = bitcast i64 %t297 to double
  %t299 = bitcast double %t298 to i64
  %t300 = alloca i64
  store i64 %t299, i64* %t300
  %t301 = load i64, i64* %t300
  %t302 = inttoptr i64 %t301 to i8*
  %t303 = bitcast i8* %t302 to %T.Bird*
  %t304 = getelementptr inbounds %T.Bird, %T.Bird* %t303, i32 0, i32 1
  %t305 = load double, double* %t304
  %t307 = ptrtoint i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.slit_306, i64 0, i64 0) to i64
  %t308 = bitcast i64 %t307 to double
  %t310 = bitcast double %t305 to i64
  %t311 = alloca i64
  store i64 %t310, i64* %t311
  %t312 = load i64, i64* %t311
  %t313 = inttoptr i64 %t312 to i8*
  %t314 = bitcast double %t308 to i64
  %t315 = alloca i64
  store i64 %t314, i64* %t315
  %t316 = load i64, i64* %t315
  %t317 = inttoptr i64 %t316 to i8*
  %t318 = call i64 @strlen(i8* %t313)
  %t319 = call i64 @strlen(i8* %t317)
  %t320 = add i64 %t318, %t319
  %t321 = add i64 %t320, 1
  %t322 = call i8* @malloc(i64 %t321)
  call void @__hulk_gc_track(i8* %t322)
  call i8* @strcpy(i8* %t322, i8* %t313)
  call i8* @strcat(i8* %t322, i8* %t317)
  %t323 = ptrtoint i8* %t322 to i64
  %t309 = bitcast i64 %t323 to double
  store double %t309, double* %t266
  br label %merge_269
merge_269:
  %t324 = load double, double* %t266
  ret double %t324
}

define i8* @Cat_new(double %name, double %indoor) {
entry:
  %t325 = call i8* @malloc(i64 40)
  call void @__hulk_gc_track(i8* %t325)
  %t326 = bitcast i8* %t325 to %T.Cat*
  %t327 = getelementptr inbounds %T.Cat, %T.Cat* %t326, i32 0, i32 0
  store i64 5, i64* %t327
  %t328 = alloca double
  store double %name, double* %t328
  %t329 = alloca double
  store double %indoor, double* %t329
  %t330 = load double, double* %t328
  %t331 = call i8* @Mammal_new(double %t330, double 4.0e0, double 1.0)
  %t332 = bitcast i8* %t331 to %T.Mammal*
  %t333 = getelementptr inbounds %T.Mammal, %T.Mammal* %t332, i32 0, i32 1
  %t334 = load double, double* %t333
  %t335 = getelementptr inbounds %T.Cat, %T.Cat* %t326, i32 0, i32 1
  store double %t334, double* %t335
  %t336 = getelementptr inbounds %T.Mammal, %T.Mammal* %t332, i32 0, i32 2
  %t337 = load double, double* %t336
  %t338 = getelementptr inbounds %T.Cat, %T.Cat* %t326, i32 0, i32 2
  store double %t337, double* %t338
  %t339 = getelementptr inbounds %T.Mammal, %T.Mammal* %t332, i32 0, i32 3
  %t340 = load double, double* %t339
  %t341 = getelementptr inbounds %T.Cat, %T.Cat* %t326, i32 0, i32 3
  store double %t340, double* %t341
  %t342 = load double, double* %t329
  %t343 = getelementptr inbounds %T.Cat, %T.Cat* %t326, i32 0, i32 4
  store double %t342, double* %t343
  ret i8* %t325
}

define double @Cat_meow(i8* %self) {
entry:
  %t344 = ptrtoint i8* %self to i64
  %t345 = bitcast i64 %t344 to double
  %t346 = bitcast double %t345 to i64
  %t347 = alloca i64
  store i64 %t346, i64* %t347
  %t348 = load i64, i64* %t347
  %t349 = inttoptr i64 %t348 to i8*
  %t350 = bitcast i8* %t349 to %T.Cat*
  %t351 = getelementptr inbounds %T.Cat, %T.Cat* %t350, i32 0, i32 1
  %t352 = load double, double* %t351
  %t354 = ptrtoint i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.slit_353, i64 0, i64 0) to i64
  %t355 = bitcast i64 %t354 to double
  %t357 = bitcast double %t352 to i64
  %t358 = alloca i64
  store i64 %t357, i64* %t358
  %t359 = load i64, i64* %t358
  %t360 = inttoptr i64 %t359 to i8*
  %t361 = bitcast double %t355 to i64
  %t362 = alloca i64
  store i64 %t361, i64* %t362
  %t363 = load i64, i64* %t362
  %t364 = inttoptr i64 %t363 to i8*
  %t365 = call i64 @strlen(i8* %t360)
  %t366 = call i64 @strlen(i8* %t364)
  %t367 = add i64 %t365, %t366
  %t368 = add i64 %t367, 1
  %t369 = call i8* @malloc(i64 %t368)
  call void @__hulk_gc_track(i8* %t369)
  call i8* @strcpy(i8* %t369, i8* %t360)
  call i8* @strcat(i8* %t369, i8* %t364)
  %t370 = ptrtoint i8* %t369 to i64
  %t356 = bitcast i64 %t370 to double
  ret double %t356
}

define double @Cat_describe(i8* %self) {
entry:
  %t371 = ptrtoint i8* %self to i64
  %t372 = bitcast i64 %t371 to double
  %t373 = bitcast double %t372 to i64
  %t374 = alloca i64
  store i64 %t373, i64* %t374
  %t375 = load i64, i64* %t374
  %t376 = inttoptr i64 %t375 to i8*
  %t377 = bitcast i8* %t376 to %T.Cat*
  %t378 = getelementptr inbounds %T.Cat, %T.Cat* %t377, i32 0, i32 4
  %t379 = load double, double* %t378
  %t380 = fcmp one double %t379, 0.0
  %t381 = alloca double
  br i1 %t380, label %then_382, label %else_383
then_382:
  %t385 = ptrtoint i8* %self to i64
  %t386 = bitcast i64 %t385 to double
  %t387 = bitcast double %t386 to i64
  %t388 = alloca i64
  store i64 %t387, i64* %t388
  %t389 = load i64, i64* %t388
  %t390 = inttoptr i64 %t389 to i8*
  %t391 = bitcast i8* %t390 to %T.Cat*
  %t392 = getelementptr inbounds %T.Cat, %T.Cat* %t391, i32 0, i32 1
  %t393 = load double, double* %t392
  %t395 = ptrtoint i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.slit_394, i64 0, i64 0) to i64
  %t396 = bitcast i64 %t395 to double
  %t398 = bitcast double %t393 to i64
  %t399 = alloca i64
  store i64 %t398, i64* %t399
  %t400 = load i64, i64* %t399
  %t401 = inttoptr i64 %t400 to i8*
  %t402 = bitcast double %t396 to i64
  %t403 = alloca i64
  store i64 %t402, i64* %t403
  %t404 = load i64, i64* %t403
  %t405 = inttoptr i64 %t404 to i8*
  %t406 = call i64 @strlen(i8* %t401)
  %t407 = call i64 @strlen(i8* %t405)
  %t408 = add i64 %t406, %t407
  %t409 = add i64 %t408, 1
  %t410 = call i8* @malloc(i64 %t409)
  call void @__hulk_gc_track(i8* %t410)
  call i8* @strcpy(i8* %t410, i8* %t401)
  call i8* @strcat(i8* %t410, i8* %t405)
  %t411 = ptrtoint i8* %t410 to i64
  %t397 = bitcast i64 %t411 to double
  store double %t397, double* %t381
  br label %merge_384
else_383:
  %t412 = ptrtoint i8* %self to i64
  %t413 = bitcast i64 %t412 to double
  %t414 = bitcast double %t413 to i64
  %t415 = alloca i64
  store i64 %t414, i64* %t415
  %t416 = load i64, i64* %t415
  %t417 = inttoptr i64 %t416 to i8*
  %t418 = bitcast i8* %t417 to %T.Cat*
  %t419 = getelementptr inbounds %T.Cat, %T.Cat* %t418, i32 0, i32 1
  %t420 = load double, double* %t419
  %t422 = ptrtoint i8* getelementptr inbounds ([15 x i8], [15 x i8]* @.slit_421, i64 0, i64 0) to i64
  %t423 = bitcast i64 %t422 to double
  %t425 = bitcast double %t420 to i64
  %t426 = alloca i64
  store i64 %t425, i64* %t426
  %t427 = load i64, i64* %t426
  %t428 = inttoptr i64 %t427 to i8*
  %t429 = bitcast double %t423 to i64
  %t430 = alloca i64
  store i64 %t429, i64* %t430
  %t431 = load i64, i64* %t430
  %t432 = inttoptr i64 %t431 to i8*
  %t433 = call i64 @strlen(i8* %t428)
  %t434 = call i64 @strlen(i8* %t432)
  %t435 = add i64 %t433, %t434
  %t436 = add i64 %t435, 1
  %t437 = call i8* @malloc(i64 %t436)
  call void @__hulk_gc_track(i8* %t437)
  call i8* @strcpy(i8* %t437, i8* %t428)
  call i8* @strcat(i8* %t437, i8* %t432)
  %t438 = ptrtoint i8* %t437 to i64
  %t424 = bitcast i64 %t438 to double
  store double %t424, double* %t381
  br label %merge_384
merge_384:
  %t439 = load double, double* %t381
  ret double %t439
}

define double @separator() {
entry:
  %t441 = ptrtoint i8* getelementptr inbounds ([97 x i8], [97 x i8]* @.slit_440, i64 0, i64 0) to i64
  %t442 = bitcast i64 %t441 to double
  %t443 = bitcast double %t442 to i64
  %t444 = alloca i64
  store i64 %t443, i64* %t444
  %t445 = load i64, i64* %t444
  %t446 = inttoptr i64 %t445 to i8*
  call i32 @puts(i8* %t446)
  ret double 0.0
}

define double @abs(double %x) {
entry:
  %t447 = alloca double
  store double %x, double* %t447
  %t448 = load double, double* %t447
  %t450 = fcmp olt double %t448, 0.0e0
  %t449 = uitofp i1 %t450 to double
  %t451 = fcmp one double %t449, 0.0
  %t452 = alloca double
  br i1 %t451, label %then_453, label %else_454
then_453:
  %t456 = load double, double* %t447
  %t457 = fsub double 0.0e0, %t456
  store double %t457, double* %t452
  br label %merge_455
else_454:
  %t458 = load double, double* %t447
  store double %t458, double* %t452
  br label %merge_455
merge_455:
  %t459 = load double, double* %t452
  ret double %t459
}

define double @max(double %a, double %b) {
entry:
  %t460 = alloca double
  store double %a, double* %t460
  %t461 = alloca double
  store double %b, double* %t461
  %t462 = load double, double* %t460
  %t463 = load double, double* %t461
  %t465 = fcmp ogt double %t462, %t463
  %t464 = uitofp i1 %t465 to double
  %t466 = fcmp one double %t464, 0.0
  %t467 = alloca double
  br i1 %t466, label %then_468, label %else_469
then_468:
  %t471 = load double, double* %t460
  store double %t471, double* %t467
  br label %merge_470
else_469:
  %t472 = load double, double* %t461
  store double %t472, double* %t467
  br label %merge_470
merge_470:
  %t473 = load double, double* %t467
  ret double %t473
}

define double @min(double %a, double %b) {
entry:
  %t474 = alloca double
  store double %a, double* %t474
  %t475 = alloca double
  store double %b, double* %t475
  %t476 = load double, double* %t474
  %t477 = load double, double* %t475
  %t479 = fcmp olt double %t476, %t477
  %t478 = uitofp i1 %t479 to double
  %t480 = fcmp one double %t478, 0.0
  %t481 = alloca double
  br i1 %t480, label %then_482, label %else_483
then_482:
  %t485 = load double, double* %t474
  store double %t485, double* %t481
  br label %merge_484
else_483:
  %t486 = load double, double* %t475
  store double %t486, double* %t481
  br label %merge_484
merge_484:
  %t487 = load double, double* %t481
  ret double %t487
}

define double @clamp(double %val, double %lo, double %hi) {
entry:
  %t488 = alloca double
  store double %val, double* %t488
  %t489 = alloca double
  store double %lo, double* %t489
  %t490 = alloca double
  store double %hi, double* %t490
  %t491 = load double, double* %t488
  %t492 = load double, double* %t489
  %t493 = call double @max(double %t491, double %t492)
  %t494 = load double, double* %t490
  %t495 = call double @min(double %t493, double %t494)
  ret double %t495
}

define double @repeat_str(double %s, double %n) {
entry:
  %t496 = alloca double
  store double %s, double* %t496
  %t497 = alloca double
  store double %n, double* %t497
  %t499 = ptrtoint i8* getelementptr inbounds ([1 x i8], [1 x i8]* @.slit_498, i64 0, i64 0) to i64
  %t500 = bitcast i64 %t499 to double
  %t501 = alloca double
  store double %t500, double* %t501
  %t502 = alloca double
  store double 0.0e0, double* %t502
  br label %wcond_503
wcond_503:
  %t506 = load double, double* %t502
  %t507 = load double, double* %t497
  %t509 = fcmp olt double %t506, %t507
  %t508 = uitofp i1 %t509 to double
  %t510 = fcmp one double %t508, 0.0
  br i1 %t510, label %wbody_504, label %wend_505
wbody_504:
  %t511 = load double, double* %t501
  %t512 = load double, double* %t496
  %t514 = bitcast double %t511 to i64
  %t515 = alloca i64
  store i64 %t514, i64* %t515
  %t516 = load i64, i64* %t515
  %t517 = inttoptr i64 %t516 to i8*
  %t518 = bitcast double %t512 to i64
  %t519 = alloca i64
  store i64 %t518, i64* %t519
  %t520 = load i64, i64* %t519
  %t521 = inttoptr i64 %t520 to i8*
  %t522 = call i64 @strlen(i8* %t517)
  %t523 = call i64 @strlen(i8* %t521)
  %t524 = add i64 %t522, %t523
  %t525 = add i64 %t524, 1
  %t526 = call i8* @malloc(i64 %t525)
  call void @__hulk_gc_track(i8* %t526)
  call i8* @strcpy(i8* %t526, i8* %t517)
  call i8* @strcat(i8* %t526, i8* %t521)
  %t527 = ptrtoint i8* %t526 to i64
  %t513 = bitcast i64 %t527 to double
  store double %t513, double* %t501
  %t528 = load double, double* %t502
  %t529 = fadd double %t528, 1.0e0
  store double %t529, double* %t502
  br label %wcond_503
wend_505:
  %t530 = load double, double* %t501
  ret double %t530
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
  %t532 = ptrtoint i8* getelementptr inbounds ([30 x i8], [30 x i8]* @.slit_531, i64 0, i64 0) to i64
  %t533 = bitcast i64 %t532 to double
  %t534 = bitcast double %t533 to i64
  %t535 = alloca i64
  store i64 %t534, i64* %t535
  %t536 = load i64, i64* %t535
  %t537 = inttoptr i64 %t536 to i8*
  call i32 @puts(i8* %t537)
  %t538 = call double @separator()
  %t540 = ptrtoint i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.slit_539, i64 0, i64 0) to i64
  %t541 = bitcast i64 %t540 to double
  %t543 = ptrtoint i8* getelementptr inbounds ([9 x i8], [9 x i8]* @.slit_542, i64 0, i64 0) to i64
  %t544 = bitcast i64 %t543 to double
  %t545 = call i8* @Dog_new(double %t541, double %t544)
  %t546 = ptrtoint i8* %t545 to i64
  %t547 = bitcast i64 %t546 to double
  %t548 = alloca double
  store double %t547, double* %t548
  %t550 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_549, i64 0, i64 0) to i64
  %t551 = bitcast i64 %t550 to double
  %t553 = ptrtoint i8* getelementptr inbounds ([7 x i8], [7 x i8]* @.slit_552, i64 0, i64 0) to i64
  %t554 = bitcast i64 %t553 to double
  %t555 = call i8* @Dog_new(double %t551, double %t554)
  %t556 = ptrtoint i8* %t555 to i64
  %t557 = bitcast i64 %t556 to double
  %t558 = alloca double
  store double %t557, double* %t558
  %t560 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_559, i64 0, i64 0) to i64
  %t561 = bitcast i64 %t560 to double
  %t562 = call i8* @Cat_new(double %t561, double 1.0)
  %t563 = ptrtoint i8* %t562 to i64
  %t564 = bitcast i64 %t563 to double
  %t565 = alloca double
  store double %t564, double* %t565
  %t567 = ptrtoint i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.slit_566, i64 0, i64 0) to i64
  %t568 = bitcast i64 %t567 to double
  %t569 = call i8* @Cat_new(double %t568, double 0.0)
  %t570 = ptrtoint i8* %t569 to i64
  %t571 = bitcast i64 %t570 to double
  %t572 = alloca double
  store double %t571, double* %t572
  %t574 = ptrtoint i8* getelementptr inbounds ([7 x i8], [7 x i8]* @.slit_573, i64 0, i64 0) to i64
  %t575 = bitcast i64 %t574 to double
  %t576 = call i8* @Bird_new(double %t575, double 1.0)
  %t577 = ptrtoint i8* %t576 to i64
  %t578 = bitcast i64 %t577 to double
  %t579 = alloca double
  store double %t578, double* %t579
  %t581 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_580, i64 0, i64 0) to i64
  %t582 = bitcast i64 %t581 to double
  %t583 = call i8* @Bird_new(double %t582, double 0.0)
  %t584 = ptrtoint i8* %t583 to i64
  %t585 = bitcast i64 %t584 to double
  %t586 = alloca double
  store double %t585, double* %t586
  %t588 = ptrtoint i8* getelementptr inbounds ([25 x i8], [25 x i8]* @.slit_587, i64 0, i64 0) to i64
  %t589 = bitcast i64 %t588 to double
  %t590 = bitcast double %t589 to i64
  %t591 = alloca i64
  store i64 %t590, i64* %t591
  %t592 = load i64, i64* %t591
  %t593 = inttoptr i64 %t592 to i8*
  call i32 @puts(i8* %t593)
  %t594 = load double, double* %t548
  %t595 = bitcast double %t594 to i64
  %t596 = alloca i64
  store i64 %t595, i64* %t596
  %t597 = load i64, i64* %t596
  %t598 = inttoptr i64 %t597 to i8*
  %t599 = call double @Animal_greet(i8* %t598)
  %t600 = bitcast double %t599 to i64
  %t601 = alloca i64
  store i64 %t600, i64* %t601
  %t602 = load i64, i64* %t601
  %t603 = inttoptr i64 %t602 to i8*
  call i32 @puts(i8* %t603)
  %t604 = load double, double* %t565
  %t605 = bitcast double %t604 to i64
  %t606 = alloca i64
  store i64 %t605, i64* %t606
  %t607 = load i64, i64* %t606
  %t608 = inttoptr i64 %t607 to i8*
  %t609 = call double @Animal_greet(i8* %t608)
  %t610 = bitcast double %t609 to i64
  %t611 = alloca i64
  store i64 %t610, i64* %t611
  %t612 = load i64, i64* %t611
  %t613 = inttoptr i64 %t612 to i8*
  call i32 @puts(i8* %t613)
  %t614 = load double, double* %t579
  %t615 = bitcast double %t614 to i64
  %t616 = alloca i64
  store i64 %t615, i64* %t616
  %t617 = load i64, i64* %t616
  %t618 = inttoptr i64 %t617 to i8*
  %t619 = call double @Animal_greet(i8* %t618)
  %t620 = bitcast double %t619 to i64
  %t621 = alloca i64
  store i64 %t620, i64* %t621
  %t622 = load i64, i64* %t621
  %t623 = inttoptr i64 %t622 to i8*
  call i32 @puts(i8* %t623)
  %t624 = call double @separator()
  %t626 = ptrtoint i8* getelementptr inbounds ([20 x i8], [20 x i8]* @.slit_625, i64 0, i64 0) to i64
  %t627 = bitcast i64 %t626 to double
  %t628 = bitcast double %t627 to i64
  %t629 = alloca i64
  store i64 %t628, i64* %t629
  %t630 = load i64, i64* %t629
  %t631 = inttoptr i64 %t630 to i8*
  call i32 @puts(i8* %t631)
  %t632 = load double, double* %t548
  %t633 = bitcast double %t632 to i64
  %t634 = alloca i64
  store i64 %t633, i64* %t634
  %t635 = load i64, i64* %t634
  %t636 = inttoptr i64 %t635 to i8*
  %t637 = call double @Dog_bark(i8* %t636)
  %t638 = bitcast double %t637 to i64
  %t639 = alloca i64
  store i64 %t638, i64* %t639
  %t640 = load i64, i64* %t639
  %t641 = inttoptr i64 %t640 to i8*
  call i32 @puts(i8* %t641)
  %t642 = load double, double* %t558
  %t643 = bitcast double %t642 to i64
  %t644 = alloca i64
  store i64 %t643, i64* %t644
  %t645 = load i64, i64* %t644
  %t646 = inttoptr i64 %t645 to i8*
  %t647 = call double @Dog_bark(i8* %t646)
  %t648 = bitcast double %t647 to i64
  %t649 = alloca i64
  store i64 %t648, i64* %t649
  %t650 = load i64, i64* %t649
  %t651 = inttoptr i64 %t650 to i8*
  call i32 @puts(i8* %t651)
  %t652 = load double, double* %t548
  %t653 = bitcast double %t652 to i64
  %t654 = alloca i64
  store i64 %t653, i64* %t654
  %t655 = load i64, i64* %t654
  %t656 = inttoptr i64 %t655 to i8*
  %t657 = call double @Dog_describe(i8* %t656)
  %t658 = bitcast double %t657 to i64
  %t659 = alloca i64
  store i64 %t658, i64* %t659
  %t660 = load i64, i64* %t659
  %t661 = inttoptr i64 %t660 to i8*
  call i32 @puts(i8* %t661)
  %t662 = load double, double* %t558
  %t663 = bitcast double %t662 to i64
  %t664 = alloca i64
  store i64 %t663, i64* %t664
  %t665 = load i64, i64* %t664
  %t666 = inttoptr i64 %t665 to i8*
  %t667 = call double @Dog_describe(i8* %t666)
  %t668 = bitcast double %t667 to i64
  %t669 = alloca i64
  store i64 %t668, i64* %t669
  %t670 = load i64, i64* %t669
  %t671 = inttoptr i64 %t670 to i8*
  call i32 @puts(i8* %t671)
  %t672 = call double @separator()
  %t674 = ptrtoint i8* getelementptr inbounds ([20 x i8], [20 x i8]* @.slit_673, i64 0, i64 0) to i64
  %t675 = bitcast i64 %t674 to double
  %t676 = bitcast double %t675 to i64
  %t677 = alloca i64
  store i64 %t676, i64* %t677
  %t678 = load i64, i64* %t677
  %t679 = inttoptr i64 %t678 to i8*
  call i32 @puts(i8* %t679)
  %t680 = load double, double* %t565
  %t681 = bitcast double %t680 to i64
  %t682 = alloca i64
  store i64 %t681, i64* %t682
  %t683 = load i64, i64* %t682
  %t684 = inttoptr i64 %t683 to i8*
  %t685 = call double @Cat_meow(i8* %t684)
  %t686 = bitcast double %t685 to i64
  %t687 = alloca i64
  store i64 %t686, i64* %t687
  %t688 = load i64, i64* %t687
  %t689 = inttoptr i64 %t688 to i8*
  call i32 @puts(i8* %t689)
  %t690 = load double, double* %t572
  %t691 = bitcast double %t690 to i64
  %t692 = alloca i64
  store i64 %t691, i64* %t692
  %t693 = load i64, i64* %t692
  %t694 = inttoptr i64 %t693 to i8*
  %t695 = call double @Cat_meow(i8* %t694)
  %t696 = bitcast double %t695 to i64
  %t697 = alloca i64
  store i64 %t696, i64* %t697
  %t698 = load i64, i64* %t697
  %t699 = inttoptr i64 %t698 to i8*
  call i32 @puts(i8* %t699)
  %t700 = load double, double* %t565
  %t701 = bitcast double %t700 to i64
  %t702 = alloca i64
  store i64 %t701, i64* %t702
  %t703 = load i64, i64* %t702
  %t704 = inttoptr i64 %t703 to i8*
  %t705 = call double @Cat_describe(i8* %t704)
  %t706 = bitcast double %t705 to i64
  %t707 = alloca i64
  store i64 %t706, i64* %t707
  %t708 = load i64, i64* %t707
  %t709 = inttoptr i64 %t708 to i8*
  call i32 @puts(i8* %t709)
  %t710 = load double, double* %t572
  %t711 = bitcast double %t710 to i64
  %t712 = alloca i64
  store i64 %t711, i64* %t712
  %t713 = load i64, i64* %t712
  %t714 = inttoptr i64 %t713 to i8*
  %t715 = call double @Cat_describe(i8* %t714)
  %t716 = bitcast double %t715 to i64
  %t717 = alloca i64
  store i64 %t716, i64* %t717
  %t718 = load i64, i64* %t717
  %t719 = inttoptr i64 %t718 to i8*
  call i32 @puts(i8* %t719)
  %t720 = call double @separator()
  %t722 = ptrtoint i8* getelementptr inbounds ([18 x i8], [18 x i8]* @.slit_721, i64 0, i64 0) to i64
  %t723 = bitcast i64 %t722 to double
  %t724 = bitcast double %t723 to i64
  %t725 = alloca i64
  store i64 %t724, i64* %t725
  %t726 = load i64, i64* %t725
  %t727 = inttoptr i64 %t726 to i8*
  call i32 @puts(i8* %t727)
  %t728 = load double, double* %t579
  %t729 = bitcast double %t728 to i64
  %t730 = alloca i64
  store i64 %t729, i64* %t730
  %t731 = load i64, i64* %t730
  %t732 = inttoptr i64 %t731 to i8*
  %t733 = call double @Bird_describe(i8* %t732)
  %t734 = bitcast double %t733 to i64
  %t735 = alloca i64
  store i64 %t734, i64* %t735
  %t736 = load i64, i64* %t735
  %t737 = inttoptr i64 %t736 to i8*
  call i32 @puts(i8* %t737)
  %t738 = load double, double* %t586
  %t739 = bitcast double %t738 to i64
  %t740 = alloca i64
  store i64 %t739, i64* %t740
  %t741 = load i64, i64* %t740
  %t742 = inttoptr i64 %t741 to i8*
  %t743 = call double @Bird_describe(i8* %t742)
  %t744 = bitcast double %t743 to i64
  %t745 = alloca i64
  store i64 %t744, i64* %t745
  %t746 = load i64, i64* %t745
  %t747 = inttoptr i64 %t746 to i8*
  call i32 @puts(i8* %t747)
  %t748 = call double @separator()
  %t750 = ptrtoint i8* getelementptr inbounds ([26 x i8], [26 x i8]* @.slit_749, i64 0, i64 0) to i64
  %t751 = bitcast i64 %t750 to double
  %t752 = bitcast double %t751 to i64
  %t753 = alloca i64
  store i64 %t752, i64* %t753
  %t754 = load i64, i64* %t753
  %t755 = inttoptr i64 %t754 to i8*
  call i32 @puts(i8* %t755)
  %t756 = load double, double* %t548
  %t757 = bitcast double %t756 to i64
  %t758 = alloca i64
  store i64 %t757, i64* %t758
  %t759 = load i64, i64* %t758
  %t760 = inttoptr i64 %t759 to i8*
  %t761 = call double @Mammal_body_temp(i8* %t760)
  %t762 = bitcast double %t761 to i64
  %t763 = alloca i64
  store i64 %t762, i64* %t763
  %t764 = load i64, i64* %t763
  %t765 = inttoptr i64 %t764 to i8*
  call i32 @puts(i8* %t765)
  %t766 = load double, double* %t565
  %t767 = bitcast double %t766 to i64
  %t768 = alloca i64
  store i64 %t767, i64* %t768
  %t769 = load i64, i64* %t768
  %t770 = inttoptr i64 %t769 to i8*
  %t771 = call double @Mammal_body_temp(i8* %t770)
  %t772 = bitcast double %t771 to i64
  %t773 = alloca i64
  store i64 %t772, i64* %t773
  %t774 = load i64, i64* %t773
  %t775 = inttoptr i64 %t774 to i8*
  call i32 @puts(i8* %t775)
  %t776 = call double @separator()
  %t778 = ptrtoint i8* getelementptr inbounds ([25 x i8], [25 x i8]* @.slit_777, i64 0, i64 0) to i64
  %t779 = bitcast i64 %t778 to double
  %t780 = bitcast double %t779 to i64
  %t781 = alloca i64
  store i64 %t780, i64* %t781
  %t782 = load i64, i64* %t781
  %t783 = inttoptr i64 %t782 to i8*
  call i32 @puts(i8* %t783)
  %t785 = ptrtoint i8* getelementptr inbounds ([15 x i8], [15 x i8]* @.slit_784, i64 0, i64 0) to i64
  %t786 = bitcast i64 %t785 to double
  %t787 = bitcast double %t786 to i64
  %t788 = alloca i64
  store i64 %t787, i64* %t788
  %t789 = load i64, i64* %t788
  %t790 = inttoptr i64 %t789 to i8*
  call i32 @puts(i8* %t790)
  %t791 = load double, double* %t548
  %t792 = bitcast double %t791 to i64
  %t793 = alloca i64
  store i64 %t792, i64* %t793
  %t794 = load i64, i64* %t793
  %t795 = inttoptr i64 %t794 to i64*
  %t796 = load i64, i64* %t795
  %t797 = alloca double
  store double 0.0, double* %t797
  %t798 = icmp eq i64 %t796, 1
  br i1 %t798, label %is_match_799, label %is_next_800
is_match_799:
  store double 1.0, double* %t797
  br label %is_next_800
is_next_800:
  %t801 = icmp eq i64 %t796, 2
  br i1 %t801, label %is_match_802, label %is_next_803
is_match_802:
  store double 1.0, double* %t797
  br label %is_next_803
is_next_803:
  %t804 = icmp eq i64 %t796, 5
  br i1 %t804, label %is_match_805, label %is_next_806
is_match_805:
  store double 1.0, double* %t797
  br label %is_next_806
is_next_806:
  %t807 = icmp eq i64 %t796, 3
  br i1 %t807, label %is_match_808, label %is_next_809
is_match_808:
  store double 1.0, double* %t797
  br label %is_next_809
is_next_809:
  %t810 = icmp eq i64 %t796, 4
  br i1 %t810, label %is_match_811, label %is_next_812
is_match_811:
  store double 1.0, double* %t797
  br label %is_next_812
is_next_812:
  %t813 = load double, double* %t797
  %t814 = fcmp one double %t813, 0.0
  %t815 = select i1 %t814, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t815)
  %t817 = ptrtoint i8* getelementptr inbounds ([15 x i8], [15 x i8]* @.slit_816, i64 0, i64 0) to i64
  %t818 = bitcast i64 %t817 to double
  %t819 = bitcast double %t818 to i64
  %t820 = alloca i64
  store i64 %t819, i64* %t820
  %t821 = load i64, i64* %t820
  %t822 = inttoptr i64 %t821 to i8*
  call i32 @puts(i8* %t822)
  %t823 = load double, double* %t548
  %t824 = bitcast double %t823 to i64
  %t825 = alloca i64
  store i64 %t824, i64* %t825
  %t826 = load i64, i64* %t825
  %t827 = inttoptr i64 %t826 to i64*
  %t828 = load i64, i64* %t827
  %t829 = alloca double
  store double 0.0, double* %t829
  %t830 = icmp eq i64 %t828, 2
  br i1 %t830, label %is_match_831, label %is_next_832
is_match_831:
  store double 1.0, double* %t829
  br label %is_next_832
is_next_832:
  %t833 = icmp eq i64 %t828, 5
  br i1 %t833, label %is_match_834, label %is_next_835
is_match_834:
  store double 1.0, double* %t829
  br label %is_next_835
is_next_835:
  %t836 = icmp eq i64 %t828, 3
  br i1 %t836, label %is_match_837, label %is_next_838
is_match_837:
  store double 1.0, double* %t829
  br label %is_next_838
is_next_838:
  %t839 = load double, double* %t829
  %t840 = fcmp one double %t839, 0.0
  %t841 = select i1 %t840, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t841)
  %t843 = ptrtoint i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.slit_842, i64 0, i64 0) to i64
  %t844 = bitcast i64 %t843 to double
  %t845 = bitcast double %t844 to i64
  %t846 = alloca i64
  store i64 %t845, i64* %t846
  %t847 = load i64, i64* %t846
  %t848 = inttoptr i64 %t847 to i8*
  call i32 @puts(i8* %t848)
  %t849 = load double, double* %t548
  %t850 = bitcast double %t849 to i64
  %t851 = alloca i64
  store i64 %t850, i64* %t851
  %t852 = load i64, i64* %t851
  %t853 = inttoptr i64 %t852 to i64*
  %t854 = load i64, i64* %t853
  %t855 = alloca double
  store double 0.0, double* %t855
  %t856 = icmp eq i64 %t854, 3
  br i1 %t856, label %is_match_857, label %is_next_858
is_match_857:
  store double 1.0, double* %t855
  br label %is_next_858
is_next_858:
  %t859 = load double, double* %t855
  %t860 = fcmp one double %t859, 0.0
  %t861 = select i1 %t860, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t861)
  %t863 = ptrtoint i8* getelementptr inbounds ([18 x i8], [18 x i8]* @.slit_862, i64 0, i64 0) to i64
  %t864 = bitcast i64 %t863 to double
  %t865 = bitcast double %t864 to i64
  %t866 = alloca i64
  store i64 %t865, i64* %t866
  %t867 = load i64, i64* %t866
  %t868 = inttoptr i64 %t867 to i8*
  call i32 @puts(i8* %t868)
  %t869 = load double, double* %t579
  %t870 = bitcast double %t869 to i64
  %t871 = alloca i64
  store i64 %t870, i64* %t871
  %t872 = load i64, i64* %t871
  %t873 = inttoptr i64 %t872 to i64*
  %t874 = load i64, i64* %t873
  %t875 = alloca double
  store double 0.0, double* %t875
  %t876 = icmp eq i64 %t874, 2
  br i1 %t876, label %is_match_877, label %is_next_878
is_match_877:
  store double 1.0, double* %t875
  br label %is_next_878
is_next_878:
  %t879 = icmp eq i64 %t874, 5
  br i1 %t879, label %is_match_880, label %is_next_881
is_match_880:
  store double 1.0, double* %t875
  br label %is_next_881
is_next_881:
  %t882 = icmp eq i64 %t874, 3
  br i1 %t882, label %is_match_883, label %is_next_884
is_match_883:
  store double 1.0, double* %t875
  br label %is_next_884
is_next_884:
  %t885 = load double, double* %t875
  %t886 = fcmp one double %t885, 0.0
  %t887 = select i1 %t886, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t887)
  %t889 = ptrtoint i8* getelementptr inbounds ([18 x i8], [18 x i8]* @.slit_888, i64 0, i64 0) to i64
  %t890 = bitcast i64 %t889 to double
  %t891 = bitcast double %t890 to i64
  %t892 = alloca i64
  store i64 %t891, i64* %t892
  %t893 = load i64, i64* %t892
  %t894 = inttoptr i64 %t893 to i8*
  call i32 @puts(i8* %t894)
  %t895 = load double, double* %t579
  %t896 = bitcast double %t895 to i64
  %t897 = alloca i64
  store i64 %t896, i64* %t897
  %t898 = load i64, i64* %t897
  %t899 = inttoptr i64 %t898 to i64*
  %t900 = load i64, i64* %t899
  %t901 = alloca double
  store double 0.0, double* %t901
  %t902 = icmp eq i64 %t900, 1
  br i1 %t902, label %is_match_903, label %is_next_904
is_match_903:
  store double 1.0, double* %t901
  br label %is_next_904
is_next_904:
  %t905 = icmp eq i64 %t900, 2
  br i1 %t905, label %is_match_906, label %is_next_907
is_match_906:
  store double 1.0, double* %t901
  br label %is_next_907
is_next_907:
  %t908 = icmp eq i64 %t900, 5
  br i1 %t908, label %is_match_909, label %is_next_910
is_match_909:
  store double 1.0, double* %t901
  br label %is_next_910
is_next_910:
  %t911 = icmp eq i64 %t900, 3
  br i1 %t911, label %is_match_912, label %is_next_913
is_match_912:
  store double 1.0, double* %t901
  br label %is_next_913
is_next_913:
  %t914 = icmp eq i64 %t900, 4
  br i1 %t914, label %is_match_915, label %is_next_916
is_match_915:
  store double 1.0, double* %t901
  br label %is_next_916
is_next_916:
  %t917 = load double, double* %t901
  %t918 = fcmp one double %t917, 0.0
  %t919 = select i1 %t918, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t919)
  %t920 = call double @separator()
  %t922 = ptrtoint i8* getelementptr inbounds ([29 x i8], [29 x i8]* @.slit_921, i64 0, i64 0) to i64
  %t923 = bitcast i64 %t922 to double
  %t924 = bitcast double %t923 to i64
  %t925 = alloca i64
  store i64 %t924, i64* %t925
  %t926 = load i64, i64* %t925
  %t927 = inttoptr i64 %t926 to i8*
  call i32 @puts(i8* %t927)
  %t928 = call i8* @malloc(i64 56)
  call void @__hulk_gc_track(i8* %t928)
  %t929 = bitcast i8* %t928 to double*
  store double 6.0e0, double* %t929
  %t930 = getelementptr double, double* %t929, i64 1
  store double 4.0e0, double* %t930
  %t931 = getelementptr double, double* %t929, i64 2
  store double 4.0e0, double* %t931
  %t932 = getelementptr double, double* %t929, i64 3
  store double 4.0e0, double* %t932
  %t933 = getelementptr double, double* %t929, i64 4
  store double 4.0e0, double* %t933
  %t934 = getelementptr double, double* %t929, i64 5
  store double 2.0e0, double* %t934
  %t935 = getelementptr double, double* %t929, i64 6
  store double 2.0e0, double* %t935
  %t936 = ptrtoint double* %t929 to i64
  %t937 = bitcast i64 %t936 to double
  %t938 = alloca double
  store double %t937, double* %t938
  %t939 = alloca double
  store double 0.0e0, double* %t939
  %t940 = load double, double* %t938
  %t941 = bitcast double %t940 to i64
  %t942 = alloca i64
  store i64 %t941, i64* %t942
  %t943 = load i64, i64* %t942
  %t944 = inttoptr i64 %t943 to double*
  %t945 = load double, double* %t944
  %t946 = fptosi double %t945 to i64
  %t947 = alloca i64
  store i64 0, i64* %t947
  br label %fcond_948
fcond_948:
  %t951 = load i64, i64* %t947
  %t952 = icmp slt i64 %t951, %t946
  br i1 %t952, label %fbody_949, label %fend_950
fbody_949:
  %t953 = load i64, i64* %t947
  %t954 = add i64 %t953, 1
  %t955 = getelementptr double, double* %t944, i64 %t954
  %t956 = load double, double* %t955
  %t957 = alloca double
  store double %t956, double* %t957
  %t958 = load double, double* %t939
  %t959 = load double, double* %t957
  %t960 = fadd double %t958, %t959
  store double %t960, double* %t939
  %t961 = load i64, i64* %t947
  %t962 = add i64 %t961, 1
  store i64 %t962, i64* %t947
  br label %fcond_948
fend_950:
  %t964 = ptrtoint i8* getelementptr inbounds ([19 x i8], [19 x i8]* @.slit_963, i64 0, i64 0) to i64
  %t965 = bitcast i64 %t964 to double
  %t966 = load double, double* %t939
  %t968 = bitcast double %t965 to i64
  %t969 = alloca i64
  store i64 %t968, i64* %t969
  %t970 = load i64, i64* %t969
  %t971 = inttoptr i64 %t970 to i8*
  %t972 = call i8* @__hulk_num_to_str(double %t966)
  %t973 = call i64 @strlen(i8* %t971)
  %t974 = call i64 @strlen(i8* %t972)
  %t975 = add i64 %t973, %t974
  %t976 = add i64 %t975, 2
  %t977 = call i8* @malloc(i64 %t976)
  call void @__hulk_gc_track(i8* %t977)
  call i8* @strcpy(i8* %t977, i8* %t971)
  call i8* @strcat(i8* %t977, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t977, i8* %t972)
  %t978 = ptrtoint i8* %t977 to i64
  %t967 = bitcast i64 %t978 to double
  %t979 = bitcast double %t967 to i64
  %t980 = alloca i64
  store i64 %t979, i64* %t980
  %t981 = load i64, i64* %t980
  %t982 = inttoptr i64 %t981 to i8*
  call i32 @puts(i8* %t982)
  %t983 = load double, double* %t938
  %t984 = bitcast double %t983 to i64
  %t985 = alloca i64
  store i64 %t984, i64* %t985
  %t986 = load i64, i64* %t985
  %t987 = inttoptr i64 %t986 to double*
  %t988 = load double, double* %t987
  %t989 = fptosi double %t988 to i64
  %t990 = add i64 %t989, 1
  %t991 = mul i64 %t990, 8
  %t992 = call i8* @malloc(i64 %t991)
  call void @__hulk_gc_track(i8* %t992)
  %t993 = bitcast i8* %t992 to double*
  store double %t988, double* %t993
  %t994 = alloca i64
  store i64 0, i64* %t994
  br label %vgc_995
vgc_995:
  %t998 = load i64, i64* %t994
  %t999 = icmp slt i64 %t998, %t989
  br i1 %t999, label %vgb_996, label %vge_997
vgb_996:
  %t1000 = add i64 %t998, 1
  %t1001 = getelementptr double, double* %t987, i64 %t1000
  %t1002 = load double, double* %t1001
  %t1003 = alloca double
  store double %t1002, double* %t1003
  %t1004 = load double, double* %t1003
  %t1005 = fmul double %t1004, 2.0e0
  %t1006 = getelementptr double, double* %t993, i64 %t1000
  store double %t1005, double* %t1006
  %t1007 = add i64 %t998, 1
  store i64 %t1007, i64* %t994
  br label %vgc_995
vge_997:
  %t1008 = ptrtoint double* %t993 to i64
  %t1009 = bitcast i64 %t1008 to double
  %t1010 = alloca double
  store double %t1009, double* %t1010
  %t1012 = ptrtoint i8* getelementptr inbounds ([20 x i8], [20 x i8]* @.slit_1011, i64 0, i64 0) to i64
  %t1013 = bitcast i64 %t1012 to double
  %t1014 = bitcast double %t1013 to i64
  %t1015 = alloca i64
  store i64 %t1014, i64* %t1015
  %t1016 = load i64, i64* %t1015
  %t1017 = inttoptr i64 %t1016 to i8*
  call i32 @puts(i8* %t1017)
  %t1018 = load double, double* %t1010
  %t1019 = bitcast double %t1018 to i64
  %t1020 = alloca i64
  store i64 %t1019, i64* %t1020
  %t1021 = load i64, i64* %t1020
  %t1022 = inttoptr i64 %t1021 to double*
  %t1023 = load double, double* %t1022
  %t1024 = fptosi double %t1023 to i64
  %t1025 = alloca i64
  store i64 0, i64* %t1025
  br label %fcond_1026
fcond_1026:
  %t1029 = load i64, i64* %t1025
  %t1030 = icmp slt i64 %t1029, %t1024
  br i1 %t1030, label %fbody_1027, label %fend_1028
fbody_1027:
  %t1031 = load i64, i64* %t1025
  %t1032 = add i64 %t1031, 1
  %t1033 = getelementptr double, double* %t1022, i64 %t1032
  %t1034 = load double, double* %t1033
  %t1035 = alloca double
  store double %t1034, double* %t1035
  %t1036 = load double, double* %t1035
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double %t1036)
  call i32 @puts(i8* getelementptr inbounds ([1 x i8], [1 x i8]* @.empty_s, i64 0, i64 0))
  %t1037 = load i64, i64* %t1025
  %t1038 = add i64 %t1037, 1
  store i64 %t1038, i64* %t1025
  br label %fcond_1026
fend_1028:
  %t1039 = call double @separator()
  %t1041 = ptrtoint i8* getelementptr inbounds ([31 x i8], [31 x i8]* @.slit_1040, i64 0, i64 0) to i64
  %t1042 = bitcast i64 %t1041 to double
  %t1043 = bitcast double %t1042 to i64
  %t1044 = alloca i64
  store i64 %t1043, i64* %t1044
  %t1045 = load i64, i64* %t1044
  %t1046 = inttoptr i64 %t1045 to i8*
  call i32 @puts(i8* %t1046)
  %t1052 = call i8* @malloc(i64 16)
  call void @__hulk_gc_track(i8* %t1052)
  %t1053 = bitcast i8* %t1052 to double*
  %t1054 = ptrtoint double (double*, double)* @__lambda_1047 to i64
  %t1055 = bitcast i64 %t1054 to double
  store double %t1055, double* %t1053
  %t1056 = getelementptr double, double* %t1053, i64 1
  store double 0.0, double* %t1056
  %t1057 = ptrtoint double* %t1053 to i64
  %t1058 = bitcast i64 %t1057 to double
  %t1059 = alloca double
  store double %t1058, double* %t1059
  %t1061 = ptrtoint i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.slit_1060, i64 0, i64 0) to i64
  %t1062 = bitcast i64 %t1061 to double
  %t1063 = load double, double* %t1059
  %t1064 = bitcast double %t1063 to i64
  %t1065 = alloca i64
  store i64 %t1064, i64* %t1065
  %t1066 = load i64, i64* %t1065
  %t1067 = inttoptr i64 %t1066 to double*
  %t1068 = load double, double* %t1067
  %t1069 = bitcast double %t1068 to i64
  %t1070 = alloca i64
  store i64 %t1069, i64* %t1070
  %t1071 = load i64, i64* %t1070
  %t1072 = inttoptr i64 %t1071 to double (double*, double)*
  %t1073 = getelementptr double, double* %t1067, i64 1
  %t1074 = load double, double* %t1073
  %t1075 = bitcast double %t1074 to i64
  %t1076 = alloca i64
  store i64 %t1075, i64* %t1076
  %t1077 = load i64, i64* %t1076
  %t1078 = inttoptr i64 %t1077 to double*
  %t1079 = call double %t1072(double* %t1078, double 5.0e0)
  %t1081 = bitcast double %t1062 to i64
  %t1082 = alloca i64
  store i64 %t1081, i64* %t1082
  %t1083 = load i64, i64* %t1082
  %t1084 = inttoptr i64 %t1083 to i8*
  %t1085 = call i8* @__hulk_num_to_str(double %t1079)
  %t1086 = call i64 @strlen(i8* %t1084)
  %t1087 = call i64 @strlen(i8* %t1085)
  %t1088 = add i64 %t1086, %t1087
  %t1089 = add i64 %t1088, 2
  %t1090 = call i8* @malloc(i64 %t1089)
  call void @__hulk_gc_track(i8* %t1090)
  call i8* @strcpy(i8* %t1090, i8* %t1084)
  call i8* @strcat(i8* %t1090, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1090, i8* %t1085)
  %t1091 = ptrtoint i8* %t1090 to i64
  %t1080 = bitcast i64 %t1091 to double
  %t1092 = bitcast double %t1080 to i64
  %t1093 = alloca i64
  store i64 %t1092, i64* %t1093
  %t1094 = load i64, i64* %t1093
  %t1095 = inttoptr i64 %t1094 to i8*
  call i32 @puts(i8* %t1095)
  %t1096 = alloca double
  store double 3.0e0, double* %t1096
  %t1101 = call i8* @malloc(i64 16)
  call void @__hulk_gc_track(i8* %t1101)
  %t1102 = bitcast i8* %t1101 to double*
  %t1103 = ptrtoint double (double*, double)* @__lambda_1097 to i64
  %t1104 = bitcast i64 %t1103 to double
  store double %t1104, double* %t1102
  %t1105 = getelementptr double, double* %t1102, i64 1
  store double 0.0, double* %t1105
  %t1106 = ptrtoint double* %t1102 to i64
  %t1107 = bitcast i64 %t1106 to double
  %t1108 = alloca double
  store double %t1107, double* %t1108
  %t1110 = ptrtoint i8* getelementptr inbounds ([17 x i8], [17 x i8]* @.slit_1109, i64 0, i64 0) to i64
  %t1111 = bitcast i64 %t1110 to double
  %t1112 = load double, double* %t1108
  %t1113 = bitcast double %t1112 to i64
  %t1114 = alloca i64
  store i64 %t1113, i64* %t1114
  %t1115 = load i64, i64* %t1114
  %t1116 = inttoptr i64 %t1115 to double*
  %t1117 = load double, double* %t1116
  %t1118 = bitcast double %t1117 to i64
  %t1119 = alloca i64
  store i64 %t1118, i64* %t1119
  %t1120 = load i64, i64* %t1119
  %t1121 = inttoptr i64 %t1120 to double (double*, double)*
  %t1122 = getelementptr double, double* %t1116, i64 1
  %t1123 = load double, double* %t1122
  %t1124 = bitcast double %t1123 to i64
  %t1125 = alloca i64
  store i64 %t1124, i64* %t1125
  %t1126 = load i64, i64* %t1125
  %t1127 = inttoptr i64 %t1126 to double*
  %t1128 = call double %t1121(double* %t1127, double 1.0e1)
  %t1130 = bitcast double %t1111 to i64
  %t1131 = alloca i64
  store i64 %t1130, i64* %t1131
  %t1132 = load i64, i64* %t1131
  %t1133 = inttoptr i64 %t1132 to i8*
  %t1134 = call i8* @__hulk_num_to_str(double %t1128)
  %t1135 = call i64 @strlen(i8* %t1133)
  %t1136 = call i64 @strlen(i8* %t1134)
  %t1137 = add i64 %t1135, %t1136
  %t1138 = add i64 %t1137, 2
  %t1139 = call i8* @malloc(i64 %t1138)
  call void @__hulk_gc_track(i8* %t1139)
  call i8* @strcpy(i8* %t1139, i8* %t1133)
  call i8* @strcat(i8* %t1139, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1139, i8* %t1134)
  %t1140 = ptrtoint i8* %t1139 to i64
  %t1129 = bitcast i64 %t1140 to double
  %t1141 = bitcast double %t1129 to i64
  %t1142 = alloca i64
  store i64 %t1141, i64* %t1142
  %t1143 = load i64, i64* %t1142
  %t1144 = inttoptr i64 %t1143 to i8*
  call i32 @puts(i8* %t1144)
  %t1146 = ptrtoint i8* getelementptr inbounds ([16 x i8], [16 x i8]* @.slit_1145, i64 0, i64 0) to i64
  %t1147 = bitcast i64 %t1146 to double
  %t1148 = load double, double* %t1108
  %t1149 = bitcast double %t1148 to i64
  %t1150 = alloca i64
  store i64 %t1149, i64* %t1150
  %t1151 = load i64, i64* %t1150
  %t1152 = inttoptr i64 %t1151 to double*
  %t1153 = load double, double* %t1152
  %t1154 = bitcast double %t1153 to i64
  %t1155 = alloca i64
  store i64 %t1154, i64* %t1155
  %t1156 = load i64, i64* %t1155
  %t1157 = inttoptr i64 %t1156 to double (double*, double)*
  %t1158 = getelementptr double, double* %t1152, i64 1
  %t1159 = load double, double* %t1158
  %t1160 = bitcast double %t1159 to i64
  %t1161 = alloca i64
  store i64 %t1160, i64* %t1161
  %t1162 = load i64, i64* %t1161
  %t1163 = inttoptr i64 %t1162 to double*
  %t1164 = call double %t1157(double* %t1163, double 7.0e0)
  %t1166 = bitcast double %t1147 to i64
  %t1167 = alloca i64
  store i64 %t1166, i64* %t1167
  %t1168 = load i64, i64* %t1167
  %t1169 = inttoptr i64 %t1168 to i8*
  %t1170 = call i8* @__hulk_num_to_str(double %t1164)
  %t1171 = call i64 @strlen(i8* %t1169)
  %t1172 = call i64 @strlen(i8* %t1170)
  %t1173 = add i64 %t1171, %t1172
  %t1174 = add i64 %t1173, 2
  %t1175 = call i8* @malloc(i64 %t1174)
  call void @__hulk_gc_track(i8* %t1175)
  call i8* @strcpy(i8* %t1175, i8* %t1169)
  call i8* @strcat(i8* %t1175, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1175, i8* %t1170)
  %t1176 = ptrtoint i8* %t1175 to i64
  %t1165 = bitcast i64 %t1176 to double
  %t1177 = bitcast double %t1165 to i64
  %t1178 = alloca i64
  store i64 %t1177, i64* %t1178
  %t1179 = load i64, i64* %t1178
  %t1180 = inttoptr i64 %t1179 to i8*
  call i32 @puts(i8* %t1180)
  %t1220 = call i8* @malloc(i64 16)
  call void @__hulk_gc_track(i8* %t1220)
  %t1221 = bitcast i8* %t1220 to double*
  %t1222 = ptrtoint double (double*, double)* @__lambda_1181 to i64
  %t1223 = bitcast i64 %t1222 to double
  store double %t1223, double* %t1221
  %t1224 = getelementptr double, double* %t1221, i64 1
  store double 0.0, double* %t1224
  %t1225 = ptrtoint double* %t1221 to i64
  %t1226 = bitcast i64 %t1225 to double
  %t1227 = alloca double
  store double %t1226, double* %t1227
  %t1229 = ptrtoint i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.slit_1228, i64 0, i64 0) to i64
  %t1230 = bitcast i64 %t1229 to double
  %t1231 = load double, double* %t1227
  %t1232 = bitcast double %t1231 to i64
  %t1233 = alloca i64
  store i64 %t1232, i64* %t1233
  %t1234 = load i64, i64* %t1233
  %t1235 = inttoptr i64 %t1234 to double*
  %t1236 = load double, double* %t1235
  %t1237 = bitcast double %t1236 to i64
  %t1238 = alloca i64
  store i64 %t1237, i64* %t1238
  %t1239 = load i64, i64* %t1238
  %t1240 = inttoptr i64 %t1239 to double (double*, double)*
  %t1241 = getelementptr double, double* %t1235, i64 1
  %t1242 = load double, double* %t1241
  %t1243 = bitcast double %t1242 to i64
  %t1244 = alloca i64
  store i64 %t1243, i64* %t1244
  %t1245 = load i64, i64* %t1244
  %t1246 = inttoptr i64 %t1245 to double*
  %t1247 = call double %t1240(double* %t1246, double %t1230)
  %t1248 = bitcast double %t1247 to i64
  %t1249 = alloca i64
  store i64 %t1248, i64* %t1249
  %t1250 = load i64, i64* %t1249
  %t1251 = inttoptr i64 %t1250 to i8*
  call i32 @puts(i8* %t1251)
  %t1253 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_1252, i64 0, i64 0) to i64
  %t1254 = bitcast i64 %t1253 to double
  %t1255 = load double, double* %t1227
  %t1256 = bitcast double %t1255 to i64
  %t1257 = alloca i64
  store i64 %t1256, i64* %t1257
  %t1258 = load i64, i64* %t1257
  %t1259 = inttoptr i64 %t1258 to double*
  %t1260 = load double, double* %t1259
  %t1261 = bitcast double %t1260 to i64
  %t1262 = alloca i64
  store i64 %t1261, i64* %t1262
  %t1263 = load i64, i64* %t1262
  %t1264 = inttoptr i64 %t1263 to double (double*, double)*
  %t1265 = getelementptr double, double* %t1259, i64 1
  %t1266 = load double, double* %t1265
  %t1267 = bitcast double %t1266 to i64
  %t1268 = alloca i64
  store i64 %t1267, i64* %t1268
  %t1269 = load i64, i64* %t1268
  %t1270 = inttoptr i64 %t1269 to double*
  %t1271 = call double %t1264(double* %t1270, double %t1254)
  %t1272 = bitcast double %t1271 to i64
  %t1273 = alloca i64
  store i64 %t1272, i64* %t1273
  %t1274 = load i64, i64* %t1273
  %t1275 = inttoptr i64 %t1274 to i8*
  call i32 @puts(i8* %t1275)
  %t1276 = call double @separator()
  %t1278 = ptrtoint i8* getelementptr inbounds ([23 x i8], [23 x i8]* @.slit_1277, i64 0, i64 0) to i64
  %t1279 = bitcast i64 %t1278 to double
  %t1280 = bitcast double %t1279 to i64
  %t1281 = alloca i64
  store i64 %t1280, i64* %t1281
  %t1282 = load i64, i64* %t1281
  %t1283 = inttoptr i64 %t1282 to i8*
  call i32 @puts(i8* %t1283)
  %t1285 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_1284, i64 0, i64 0) to i64
  %t1286 = bitcast i64 %t1285 to double
  %t1288 = bitcast double %t1286 to i64
  %t1289 = alloca i64
  store i64 %t1288, i64* %t1289
  %t1290 = load i64, i64* %t1289
  %t1291 = inttoptr i64 %t1290 to i8*
  %t1292 = call i8* @__hulk_num_to_str(double 3.141592653589793e0)
  %t1293 = call i64 @strlen(i8* %t1291)
  %t1294 = call i64 @strlen(i8* %t1292)
  %t1295 = add i64 %t1293, %t1294
  %t1296 = add i64 %t1295, 2
  %t1297 = call i8* @malloc(i64 %t1296)
  call void @__hulk_gc_track(i8* %t1297)
  call i8* @strcpy(i8* %t1297, i8* %t1291)
  call i8* @strcat(i8* %t1297, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1297, i8* %t1292)
  %t1298 = ptrtoint i8* %t1297 to i64
  %t1287 = bitcast i64 %t1298 to double
  %t1299 = bitcast double %t1287 to i64
  %t1300 = alloca i64
  store i64 %t1299, i64* %t1300
  %t1301 = load i64, i64* %t1300
  %t1302 = inttoptr i64 %t1301 to i8*
  call i32 @puts(i8* %t1302)
  %t1304 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_1303, i64 0, i64 0) to i64
  %t1305 = bitcast i64 %t1304 to double
  %t1307 = bitcast double %t1305 to i64
  %t1308 = alloca i64
  store i64 %t1307, i64* %t1308
  %t1309 = load i64, i64* %t1308
  %t1310 = inttoptr i64 %t1309 to i8*
  %t1311 = call i8* @__hulk_num_to_str(double 2.718281828459045e0)
  %t1312 = call i64 @strlen(i8* %t1310)
  %t1313 = call i64 @strlen(i8* %t1311)
  %t1314 = add i64 %t1312, %t1313
  %t1315 = add i64 %t1314, 2
  %t1316 = call i8* @malloc(i64 %t1315)
  call void @__hulk_gc_track(i8* %t1316)
  call i8* @strcpy(i8* %t1316, i8* %t1310)
  call i8* @strcat(i8* %t1316, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1316, i8* %t1311)
  %t1317 = ptrtoint i8* %t1316 to i64
  %t1306 = bitcast i64 %t1317 to double
  %t1318 = bitcast double %t1306 to i64
  %t1319 = alloca i64
  store i64 %t1318, i64* %t1319
  %t1320 = load i64, i64* %t1319
  %t1321 = inttoptr i64 %t1320 to i8*
  call i32 @puts(i8* %t1321)
  %t1323 = ptrtoint i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.slit_1322, i64 0, i64 0) to i64
  %t1324 = bitcast i64 %t1323 to double
  %t1326 = bitcast double %t1324 to i64
  %t1327 = alloca i64
  store i64 %t1326, i64* %t1327
  %t1328 = load i64, i64* %t1327
  %t1329 = inttoptr i64 %t1328 to i8*
  %t1330 = call i8* @__hulk_num_to_str(double 1.2e1)
  %t1331 = call i64 @strlen(i8* %t1329)
  %t1332 = call i64 @strlen(i8* %t1330)
  %t1333 = add i64 %t1331, %t1332
  %t1334 = add i64 %t1333, 2
  %t1335 = call i8* @malloc(i64 %t1334)
  call void @__hulk_gc_track(i8* %t1335)
  call i8* @strcpy(i8* %t1335, i8* %t1329)
  call i8* @strcat(i8* %t1335, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1335, i8* %t1330)
  %t1336 = ptrtoint i8* %t1335 to i64
  %t1325 = bitcast i64 %t1336 to double
  %t1337 = bitcast double %t1325 to i64
  %t1338 = alloca i64
  store i64 %t1337, i64* %t1338
  %t1339 = load i64, i64* %t1338
  %t1340 = inttoptr i64 %t1339 to i8*
  call i32 @puts(i8* %t1340)
  %t1342 = ptrtoint i8* getelementptr inbounds ([9 x i8], [9 x i8]* @.slit_1341, i64 0, i64 0) to i64
  %t1343 = bitcast i64 %t1342 to double
  %t1345 = bitcast double %t1343 to i64
  %t1346 = alloca i64
  store i64 %t1345, i64* %t1346
  %t1347 = load i64, i64* %t1346
  %t1348 = inttoptr i64 %t1347 to i8*
  %t1349 = call i8* @__hulk_num_to_str(double 0.0e0)
  %t1350 = call i64 @strlen(i8* %t1348)
  %t1351 = call i64 @strlen(i8* %t1349)
  %t1352 = add i64 %t1350, %t1351
  %t1353 = add i64 %t1352, 2
  %t1354 = call i8* @malloc(i64 %t1353)
  call void @__hulk_gc_track(i8* %t1354)
  call i8* @strcpy(i8* %t1354, i8* %t1348)
  call i8* @strcat(i8* %t1354, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1354, i8* %t1349)
  %t1355 = ptrtoint i8* %t1354 to i64
  %t1344 = bitcast i64 %t1355 to double
  %t1356 = bitcast double %t1344 to i64
  %t1357 = alloca i64
  store i64 %t1356, i64* %t1357
  %t1358 = load i64, i64* %t1357
  %t1359 = inttoptr i64 %t1358 to i8*
  call i32 @puts(i8* %t1359)
  %t1361 = ptrtoint i8* getelementptr inbounds ([9 x i8], [9 x i8]* @.slit_1360, i64 0, i64 0) to i64
  %t1362 = bitcast i64 %t1361 to double
  %t1364 = bitcast double %t1362 to i64
  %t1365 = alloca i64
  store i64 %t1364, i64* %t1365
  %t1366 = load i64, i64* %t1365
  %t1367 = inttoptr i64 %t1366 to i8*
  %t1368 = call i8* @__hulk_num_to_str(double 1.0e0)
  %t1369 = call i64 @strlen(i8* %t1367)
  %t1370 = call i64 @strlen(i8* %t1368)
  %t1371 = add i64 %t1369, %t1370
  %t1372 = add i64 %t1371, 2
  %t1373 = call i8* @malloc(i64 %t1372)
  call void @__hulk_gc_track(i8* %t1373)
  call i8* @strcpy(i8* %t1373, i8* %t1367)
  call i8* @strcat(i8* %t1373, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1373, i8* %t1368)
  %t1374 = ptrtoint i8* %t1373 to i64
  %t1363 = bitcast i64 %t1374 to double
  %t1375 = bitcast double %t1363 to i64
  %t1376 = alloca i64
  store i64 %t1375, i64* %t1376
  %t1377 = load i64, i64* %t1376
  %t1378 = inttoptr i64 %t1377 to i8*
  call i32 @puts(i8* %t1378)
  %t1380 = ptrtoint i8* getelementptr inbounds ([9 x i8], [9 x i8]* @.slit_1379, i64 0, i64 0) to i64
  %t1381 = bitcast i64 %t1380 to double
  %t1383 = bitcast double %t1381 to i64
  %t1384 = alloca i64
  store i64 %t1383, i64* %t1384
  %t1385 = load i64, i64* %t1384
  %t1386 = inttoptr i64 %t1385 to i8*
  %t1387 = call i8* @__hulk_num_to_str(double 2.718281828459045e0)
  %t1388 = call i64 @strlen(i8* %t1386)
  %t1389 = call i64 @strlen(i8* %t1387)
  %t1390 = add i64 %t1388, %t1389
  %t1391 = add i64 %t1390, 2
  %t1392 = call i8* @malloc(i64 %t1391)
  call void @__hulk_gc_track(i8* %t1392)
  call i8* @strcpy(i8* %t1392, i8* %t1386)
  call i8* @strcat(i8* %t1392, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1392, i8* %t1387)
  %t1393 = ptrtoint i8* %t1392 to i64
  %t1382 = bitcast i64 %t1393 to double
  %t1394 = bitcast double %t1382 to i64
  %t1395 = alloca i64
  store i64 %t1394, i64* %t1395
  %t1396 = load i64, i64* %t1395
  %t1397 = inttoptr i64 %t1396 to i8*
  call i32 @puts(i8* %t1397)
  %t1399 = ptrtoint i8* getelementptr inbounds ([16 x i8], [16 x i8]* @.slit_1398, i64 0, i64 0) to i64
  %t1400 = bitcast i64 %t1399 to double
  %t1402 = bitcast double %t1400 to i64
  %t1403 = alloca i64
  store i64 %t1402, i64* %t1403
  %t1404 = load i64, i64* %t1403
  %t1405 = inttoptr i64 %t1404 to i8*
  %t1406 = call i8* @__hulk_num_to_str(double 2.9999999999999996e0)
  %t1407 = call i64 @strlen(i8* %t1405)
  %t1408 = call i64 @strlen(i8* %t1406)
  %t1409 = add i64 %t1407, %t1408
  %t1410 = add i64 %t1409, 2
  %t1411 = call i8* @malloc(i64 %t1410)
  call void @__hulk_gc_track(i8* %t1411)
  call i8* @strcpy(i8* %t1411, i8* %t1405)
  call i8* @strcat(i8* %t1411, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1411, i8* %t1406)
  %t1412 = ptrtoint i8* %t1411 to i64
  %t1401 = bitcast i64 %t1412 to double
  %t1413 = bitcast double %t1401 to i64
  %t1414 = alloca i64
  store i64 %t1413, i64* %t1414
  %t1415 = load i64, i64* %t1414
  %t1416 = inttoptr i64 %t1415 to i8*
  call i32 @puts(i8* %t1416)
  %t1417 = load i1, i1* @.rand_seeded
  br i1 %t1417, label %rand_call_1419, label %rand_seed_1418
rand_seed_1418:
  %t1420 = call i64 @time(i64* null)
  %t1421 = trunc i64 %t1420 to i32
  call void @srand(i32 %t1421)
  store i1 true, i1* @.rand_seeded
  br label %rand_call_1419
rand_call_1419:
  %t1422 = call i32 @rand()
  %t1423 = sitofp i32 %t1422 to double
  %t1424 = fdiv double %t1423, 2.147483647e9
  %t1425 = alloca double
  store double %t1424, double* %t1425
  %t1427 = ptrtoint i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.slit_1426, i64 0, i64 0) to i64
  %t1428 = bitcast i64 %t1427 to double
  %t1429 = bitcast double %t1428 to i64
  %t1430 = alloca i64
  store i64 %t1429, i64* %t1430
  %t1431 = load i64, i64* %t1430
  %t1432 = inttoptr i64 %t1431 to i8*
  call i32 @puts(i8* %t1432)
  %t1433 = load double, double* %t1425
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double %t1433)
  call i32 @puts(i8* getelementptr inbounds ([1 x i8], [1 x i8]* @.empty_s, i64 0, i64 0))
  %t1434 = call double @separator()
  %t1436 = ptrtoint i8* getelementptr inbounds ([31 x i8], [31 x i8]* @.slit_1435, i64 0, i64 0) to i64
  %t1437 = bitcast i64 %t1436 to double
  %t1438 = bitcast double %t1437 to i64
  %t1439 = alloca i64
  store i64 %t1438, i64* %t1439
  %t1440 = load i64, i64* %t1439
  %t1441 = inttoptr i64 %t1440 to i8*
  call i32 @puts(i8* %t1441)
  %t1442 = alloca double
  store double 1.0e1, double* %t1442
  %t1443 = alloca double
  store double 0.0e0, double* %t1443
  %t1444 = alloca double
  store double 1.0e0, double* %t1444
  %t1445 = alloca double
  store double 0.0e0, double* %t1445
  br label %wcond_1446
wcond_1446:
  %t1449 = load double, double* %t1445
  %t1451 = fcmp olt double %t1449, 1.0e1
  %t1450 = uitofp i1 %t1451 to double
  %t1452 = fcmp one double %t1450, 0.0
  br i1 %t1452, label %wbody_1447, label %wend_1448
wbody_1447:
  %t1453 = load double, double* %t1443
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double %t1453)
  call i32 @puts(i8* getelementptr inbounds ([1 x i8], [1 x i8]* @.empty_s, i64 0, i64 0))
  %t1454 = load double, double* %t1444
  %t1455 = alloca double
  store double %t1454, double* %t1455
  %t1456 = load double, double* %t1443
  %t1457 = load double, double* %t1444
  %t1458 = fadd double %t1456, %t1457
  store double %t1458, double* %t1444
  %t1459 = load double, double* %t1455
  store double %t1459, double* %t1443
  %t1460 = load double, double* %t1445
  %t1461 = fadd double %t1460, 1.0e0
  store double %t1461, double* %t1445
  br label %wcond_1446
wend_1448:
  %t1462 = call double @separator()
  %t1464 = ptrtoint i8* getelementptr inbounds ([26 x i8], [26 x i8]* @.slit_1463, i64 0, i64 0) to i64
  %t1465 = bitcast i64 %t1464 to double
  %t1466 = bitcast double %t1465 to i64
  %t1467 = alloca i64
  store i64 %t1466, i64* %t1467
  %t1468 = load i64, i64* %t1467
  %t1469 = inttoptr i64 %t1468 to i8*
  call i32 @puts(i8* %t1469)
  %t1470 = alloca double
  store double 0.0e0, double* %t1470
  %t1471 = alloca double
  store double 1.0e0, double* %t1471
  br label %wcond_1472
wcond_1472:
  %t1475 = load double, double* %t1470
  %t1477 = fcmp olt double %t1475, 5.0e0
  %t1476 = uitofp i1 %t1477 to double
  %t1478 = fcmp one double %t1476, 0.0
  br i1 %t1478, label %wbody_1473, label %wend_1474
wbody_1473:
  %t1480 = ptrtoint i8* getelementptr inbounds ([10 x i8], [10 x i8]* @.slit_1479, i64 0, i64 0) to i64
  %t1481 = bitcast i64 %t1480 to double
  %t1482 = load double, double* %t1470
  %t1484 = bitcast double %t1481 to i64
  %t1485 = alloca i64
  store i64 %t1484, i64* %t1485
  %t1486 = load i64, i64* %t1485
  %t1487 = inttoptr i64 %t1486 to i8*
  %t1488 = call i8* @__hulk_num_to_str(double %t1482)
  %t1489 = call i64 @strlen(i8* %t1487)
  %t1490 = call i64 @strlen(i8* %t1488)
  %t1491 = add i64 %t1489, %t1490
  %t1492 = add i64 %t1491, 2
  %t1493 = call i8* @malloc(i64 %t1492)
  call void @__hulk_gc_track(i8* %t1493)
  call i8* @strcpy(i8* %t1493, i8* %t1487)
  call i8* @strcat(i8* %t1493, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1493, i8* %t1488)
  %t1494 = ptrtoint i8* %t1493 to i64
  %t1483 = bitcast i64 %t1494 to double
  %t1495 = bitcast double %t1483 to i64
  %t1496 = alloca i64
  store i64 %t1495, i64* %t1496
  %t1497 = load i64, i64* %t1496
  %t1498 = inttoptr i64 %t1497 to i8*
  call i32 @puts(i8* %t1498)
  %t1499 = load double, double* %t1470
  %t1500 = load double, double* %t1471
  %t1501 = fadd double %t1499, %t1500
  store double %t1501, double* %t1470
  %t1502 = load double, double* %t1471
  %t1503 = fadd double %t1502, 1.0e0
  store double %t1503, double* %t1471
  br label %wcond_1472
wend_1474:
  %t1505 = ptrtoint i8* getelementptr inbounds ([16 x i8], [16 x i8]* @.slit_1504, i64 0, i64 0) to i64
  %t1506 = bitcast i64 %t1505 to double
  %t1507 = load double, double* %t1470
  %t1509 = bitcast double %t1506 to i64
  %t1510 = alloca i64
  store i64 %t1509, i64* %t1510
  %t1511 = load i64, i64* %t1510
  %t1512 = inttoptr i64 %t1511 to i8*
  %t1513 = call i8* @__hulk_num_to_str(double %t1507)
  %t1514 = call i64 @strlen(i8* %t1512)
  %t1515 = call i64 @strlen(i8* %t1513)
  %t1516 = add i64 %t1514, %t1515
  %t1517 = add i64 %t1516, 2
  %t1518 = call i8* @malloc(i64 %t1517)
  call void @__hulk_gc_track(i8* %t1518)
  call i8* @strcpy(i8* %t1518, i8* %t1512)
  call i8* @strcat(i8* %t1518, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1518, i8* %t1513)
  %t1519 = ptrtoint i8* %t1518 to i64
  %t1508 = bitcast i64 %t1519 to double
  %t1520 = bitcast double %t1508 to i64
  %t1521 = alloca i64
  store i64 %t1520, i64* %t1521
  %t1522 = load i64, i64* %t1521
  %t1523 = inttoptr i64 %t1522 to i8*
  call i32 @puts(i8* %t1523)
  %t1524 = call double @separator()
  %t1526 = ptrtoint i8* getelementptr inbounds ([26 x i8], [26 x i8]* @.slit_1525, i64 0, i64 0) to i64
  %t1527 = bitcast i64 %t1526 to double
  %t1528 = bitcast double %t1527 to i64
  %t1529 = alloca i64
  store i64 %t1528, i64* %t1529
  %t1530 = load i64, i64* %t1529
  %t1531 = inttoptr i64 %t1530 to i8*
  call i32 @puts(i8* %t1531)
  %t1533 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_1532, i64 0, i64 0) to i64
  %t1534 = bitcast i64 %t1533 to double
  %t1535 = alloca double
  store double %t1534, double* %t1535
  %t1537 = ptrtoint i8* getelementptr inbounds ([9 x i8], [9 x i8]* @.slit_1536, i64 0, i64 0) to i64
  %t1538 = bitcast i64 %t1537 to double
  %t1539 = alloca double
  store double %t1538, double* %t1539
  %t1541 = ptrtoint i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.slit_1540, i64 0, i64 0) to i64
  %t1542 = bitcast i64 %t1541 to double
  %t1543 = alloca double
  store double %t1542, double* %t1543
  %t1545 = ptrtoint i8* getelementptr inbounds ([19 x i8], [19 x i8]* @.slit_1544, i64 0, i64 0) to i64
  %t1546 = bitcast i64 %t1545 to double
  %t1547 = bitcast double %t1546 to i64
  %t1548 = alloca i64
  store i64 %t1547, i64* %t1548
  %t1549 = load i64, i64* %t1548
  %t1550 = inttoptr i64 %t1549 to i8*
  call i32 @puts(i8* %t1550)
  %t1552 = ptrtoint i8* getelementptr inbounds ([18 x i8], [18 x i8]* @.slit_1551, i64 0, i64 0) to i64
  %t1553 = bitcast i64 %t1552 to double
  %t1554 = bitcast double %t1553 to i64
  %t1555 = alloca i64
  store i64 %t1554, i64* %t1555
  %t1556 = load i64, i64* %t1555
  %t1557 = inttoptr i64 %t1556 to i8*
  call i32 @puts(i8* %t1557)
  %t1559 = ptrtoint i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.slit_1558, i64 0, i64 0) to i64
  %t1560 = bitcast i64 %t1559 to double
  %t1561 = call double @repeat_str(double %t1560, double 1.0e1)
  %t1562 = alloca double
  store double %t1561, double* %t1562
  %t1563 = load double, double* %t1562
  %t1565 = ptrtoint i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.slit_1564, i64 0, i64 0) to i64
  %t1566 = bitcast i64 %t1565 to double
  %t1568 = bitcast double %t1563 to i64
  %t1569 = alloca i64
  store i64 %t1568, i64* %t1569
  %t1570 = load i64, i64* %t1569
  %t1571 = inttoptr i64 %t1570 to i8*
  %t1572 = bitcast double %t1566 to i64
  %t1573 = alloca i64
  store i64 %t1572, i64* %t1573
  %t1574 = load i64, i64* %t1573
  %t1575 = inttoptr i64 %t1574 to i8*
  %t1576 = call i64 @strlen(i8* %t1571)
  %t1577 = call i64 @strlen(i8* %t1575)
  %t1578 = add i64 %t1576, %t1577
  %t1579 = add i64 %t1578, 1
  %t1580 = call i8* @malloc(i64 %t1579)
  call void @__hulk_gc_track(i8* %t1580)
  call i8* @strcpy(i8* %t1580, i8* %t1571)
  call i8* @strcat(i8* %t1580, i8* %t1575)
  %t1581 = ptrtoint i8* %t1580 to i64
  %t1567 = bitcast i64 %t1581 to double
  %t1583 = ptrtoint i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.slit_1582, i64 0, i64 0) to i64
  %t1584 = bitcast i64 %t1583 to double
  %t1586 = bitcast double %t1567 to i64
  %t1587 = alloca i64
  store i64 %t1586, i64* %t1587
  %t1588 = load i64, i64* %t1587
  %t1589 = inttoptr i64 %t1588 to i8*
  %t1590 = bitcast double %t1584 to i64
  %t1591 = alloca i64
  store i64 %t1590, i64* %t1591
  %t1592 = load i64, i64* %t1591
  %t1593 = inttoptr i64 %t1592 to i8*
  %t1594 = call i64 @strlen(i8* %t1589)
  %t1595 = call i64 @strlen(i8* %t1593)
  %t1596 = add i64 %t1594, %t1595
  %t1597 = add i64 %t1596, 1
  %t1598 = call i8* @malloc(i64 %t1597)
  call void @__hulk_gc_track(i8* %t1598)
  call i8* @strcpy(i8* %t1598, i8* %t1589)
  call i8* @strcat(i8* %t1598, i8* %t1593)
  %t1599 = ptrtoint i8* %t1598 to i64
  %t1585 = bitcast i64 %t1599 to double
  %t1601 = ptrtoint i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.slit_1600, i64 0, i64 0) to i64
  %t1602 = bitcast i64 %t1601 to double
  %t1604 = bitcast double %t1585 to i64
  %t1605 = alloca i64
  store i64 %t1604, i64* %t1605
  %t1606 = load i64, i64* %t1605
  %t1607 = inttoptr i64 %t1606 to i8*
  %t1608 = bitcast double %t1602 to i64
  %t1609 = alloca i64
  store i64 %t1608, i64* %t1609
  %t1610 = load i64, i64* %t1609
  %t1611 = inttoptr i64 %t1610 to i8*
  %t1612 = call i64 @strlen(i8* %t1607)
  %t1613 = call i64 @strlen(i8* %t1611)
  %t1614 = add i64 %t1612, %t1613
  %t1615 = add i64 %t1614, 1
  %t1616 = call i8* @malloc(i64 %t1615)
  call void @__hulk_gc_track(i8* %t1616)
  call i8* @strcpy(i8* %t1616, i8* %t1607)
  call i8* @strcat(i8* %t1616, i8* %t1611)
  %t1617 = ptrtoint i8* %t1616 to i64
  %t1603 = bitcast i64 %t1617 to double
  %t1618 = load double, double* %t1562
  %t1620 = bitcast double %t1603 to i64
  %t1621 = alloca i64
  store i64 %t1620, i64* %t1621
  %t1622 = load i64, i64* %t1621
  %t1623 = inttoptr i64 %t1622 to i8*
  %t1624 = bitcast double %t1618 to i64
  %t1625 = alloca i64
  store i64 %t1624, i64* %t1625
  %t1626 = load i64, i64* %t1625
  %t1627 = inttoptr i64 %t1626 to i8*
  %t1628 = call i64 @strlen(i8* %t1623)
  %t1629 = call i64 @strlen(i8* %t1627)
  %t1630 = add i64 %t1628, %t1629
  %t1631 = add i64 %t1630, 1
  %t1632 = call i8* @malloc(i64 %t1631)
  call void @__hulk_gc_track(i8* %t1632)
  call i8* @strcpy(i8* %t1632, i8* %t1623)
  call i8* @strcat(i8* %t1632, i8* %t1627)
  %t1633 = ptrtoint i8* %t1632 to i64
  %t1619 = bitcast i64 %t1633 to double
  %t1634 = bitcast double %t1619 to i64
  %t1635 = alloca i64
  store i64 %t1634, i64* %t1635
  %t1636 = load i64, i64* %t1635
  %t1637 = inttoptr i64 %t1636 to i8*
  call i32 @puts(i8* %t1637)
  %t1638 = call double @separator()
  %t1640 = ptrtoint i8* getelementptr inbounds ([26 x i8], [26 x i8]* @.slit_1639, i64 0, i64 0) to i64
  %t1641 = bitcast i64 %t1640 to double
  %t1642 = bitcast double %t1641 to i64
  %t1643 = alloca i64
  store i64 %t1642, i64* %t1643
  %t1644 = load i64, i64* %t1643
  %t1645 = inttoptr i64 %t1644 to i8*
  call i32 @puts(i8* %t1645)
  %t1647 = ptrtoint i8* getelementptr inbounds ([11 x i8], [11 x i8]* @.slit_1646, i64 0, i64 0) to i64
  %t1648 = bitcast i64 %t1647 to double
  %t1649 = call double @abs(double -4.2e1)
  %t1651 = bitcast double %t1648 to i64
  %t1652 = alloca i64
  store i64 %t1651, i64* %t1652
  %t1653 = load i64, i64* %t1652
  %t1654 = inttoptr i64 %t1653 to i8*
  %t1655 = call i8* @__hulk_num_to_str(double %t1649)
  %t1656 = call i64 @strlen(i8* %t1654)
  %t1657 = call i64 @strlen(i8* %t1655)
  %t1658 = add i64 %t1656, %t1657
  %t1659 = add i64 %t1658, 2
  %t1660 = call i8* @malloc(i64 %t1659)
  call void @__hulk_gc_track(i8* %t1660)
  call i8* @strcpy(i8* %t1660, i8* %t1654)
  call i8* @strcat(i8* %t1660, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1660, i8* %t1655)
  %t1661 = ptrtoint i8* %t1660 to i64
  %t1650 = bitcast i64 %t1661 to double
  %t1662 = bitcast double %t1650 to i64
  %t1663 = alloca i64
  store i64 %t1662, i64* %t1663
  %t1664 = load i64, i64* %t1663
  %t1665 = inttoptr i64 %t1664 to i8*
  call i32 @puts(i8* %t1665)
  %t1667 = ptrtoint i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.slit_1666, i64 0, i64 0) to i64
  %t1668 = bitcast i64 %t1667 to double
  %t1669 = call double @max(double 1.0e1, double 2.0e1)
  %t1671 = bitcast double %t1668 to i64
  %t1672 = alloca i64
  store i64 %t1671, i64* %t1672
  %t1673 = load i64, i64* %t1672
  %t1674 = inttoptr i64 %t1673 to i8*
  %t1675 = call i8* @__hulk_num_to_str(double %t1669)
  %t1676 = call i64 @strlen(i8* %t1674)
  %t1677 = call i64 @strlen(i8* %t1675)
  %t1678 = add i64 %t1676, %t1677
  %t1679 = add i64 %t1678, 2
  %t1680 = call i8* @malloc(i64 %t1679)
  call void @__hulk_gc_track(i8* %t1680)
  call i8* @strcpy(i8* %t1680, i8* %t1674)
  call i8* @strcat(i8* %t1680, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1680, i8* %t1675)
  %t1681 = ptrtoint i8* %t1680 to i64
  %t1670 = bitcast i64 %t1681 to double
  %t1682 = bitcast double %t1670 to i64
  %t1683 = alloca i64
  store i64 %t1682, i64* %t1683
  %t1684 = load i64, i64* %t1683
  %t1685 = inttoptr i64 %t1684 to i8*
  call i32 @puts(i8* %t1685)
  %t1687 = ptrtoint i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.slit_1686, i64 0, i64 0) to i64
  %t1688 = bitcast i64 %t1687 to double
  %t1689 = call double @min(double 1.0e1, double 2.0e1)
  %t1691 = bitcast double %t1688 to i64
  %t1692 = alloca i64
  store i64 %t1691, i64* %t1692
  %t1693 = load i64, i64* %t1692
  %t1694 = inttoptr i64 %t1693 to i8*
  %t1695 = call i8* @__hulk_num_to_str(double %t1689)
  %t1696 = call i64 @strlen(i8* %t1694)
  %t1697 = call i64 @strlen(i8* %t1695)
  %t1698 = add i64 %t1696, %t1697
  %t1699 = add i64 %t1698, 2
  %t1700 = call i8* @malloc(i64 %t1699)
  call void @__hulk_gc_track(i8* %t1700)
  call i8* @strcpy(i8* %t1700, i8* %t1694)
  call i8* @strcat(i8* %t1700, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1700, i8* %t1695)
  %t1701 = ptrtoint i8* %t1700 to i64
  %t1690 = bitcast i64 %t1701 to double
  %t1702 = bitcast double %t1690 to i64
  %t1703 = alloca i64
  store i64 %t1702, i64* %t1703
  %t1704 = load i64, i64* %t1703
  %t1705 = inttoptr i64 %t1704 to i8*
  call i32 @puts(i8* %t1705)
  %t1707 = ptrtoint i8* getelementptr inbounds ([21 x i8], [21 x i8]* @.slit_1706, i64 0, i64 0) to i64
  %t1708 = bitcast i64 %t1707 to double
  %t1709 = call double @clamp(double 1.5e2, double 0.0e0, double 1.0e2)
  %t1711 = bitcast double %t1708 to i64
  %t1712 = alloca i64
  store i64 %t1711, i64* %t1712
  %t1713 = load i64, i64* %t1712
  %t1714 = inttoptr i64 %t1713 to i8*
  %t1715 = call i8* @__hulk_num_to_str(double %t1709)
  %t1716 = call i64 @strlen(i8* %t1714)
  %t1717 = call i64 @strlen(i8* %t1715)
  %t1718 = add i64 %t1716, %t1717
  %t1719 = add i64 %t1718, 2
  %t1720 = call i8* @malloc(i64 %t1719)
  call void @__hulk_gc_track(i8* %t1720)
  call i8* @strcpy(i8* %t1720, i8* %t1714)
  call i8* @strcat(i8* %t1720, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1720, i8* %t1715)
  %t1721 = ptrtoint i8* %t1720 to i64
  %t1710 = bitcast i64 %t1721 to double
  %t1722 = bitcast double %t1710 to i64
  %t1723 = alloca i64
  store i64 %t1722, i64* %t1723
  %t1724 = load i64, i64* %t1723
  %t1725 = inttoptr i64 %t1724 to i8*
  call i32 @puts(i8* %t1725)
  %t1727 = ptrtoint i8* getelementptr inbounds ([20 x i8], [20 x i8]* @.slit_1726, i64 0, i64 0) to i64
  %t1728 = bitcast i64 %t1727 to double
  %t1729 = call double @clamp(double -5.0e0, double 0.0e0, double 1.0e2)
  %t1731 = bitcast double %t1728 to i64
  %t1732 = alloca i64
  store i64 %t1731, i64* %t1732
  %t1733 = load i64, i64* %t1732
  %t1734 = inttoptr i64 %t1733 to i8*
  %t1735 = call i8* @__hulk_num_to_str(double %t1729)
  %t1736 = call i64 @strlen(i8* %t1734)
  %t1737 = call i64 @strlen(i8* %t1735)
  %t1738 = add i64 %t1736, %t1737
  %t1739 = add i64 %t1738, 2
  %t1740 = call i8* @malloc(i64 %t1739)
  call void @__hulk_gc_track(i8* %t1740)
  call i8* @strcpy(i8* %t1740, i8* %t1734)
  call i8* @strcat(i8* %t1740, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))
  call i8* @strcat(i8* %t1740, i8* %t1735)
  %t1741 = ptrtoint i8* %t1740 to i64
  %t1730 = bitcast i64 %t1741 to double
  %t1742 = bitcast double %t1730 to i64
  %t1743 = alloca i64
  store i64 %t1742, i64* %t1743
  %t1744 = load i64, i64* %t1743
  %t1745 = inttoptr i64 %t1744 to i8*
  call i32 @puts(i8* %t1745)
  %t1746 = call double @separator()
  %t1748 = ptrtoint i8* getelementptr inbounds ([22 x i8], [22 x i8]* @.slit_1747, i64 0, i64 0) to i64
  %t1749 = bitcast i64 %t1748 to double
  %t1750 = bitcast double %t1749 to i64
  %t1751 = alloca i64
  store i64 %t1750, i64* %t1751
  %t1752 = load i64, i64* %t1751
  %t1753 = inttoptr i64 %t1752 to i8*
  call i32 @puts(i8* %t1753)
  %t1755 = ptrtoint i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.slit_1754, i64 0, i64 0) to i64
  %t1756 = bitcast i64 %t1755 to double
  %t1757 = bitcast double %t1756 to i64
  %t1758 = alloca i64
  store i64 %t1757, i64* %t1758
  %t1759 = load i64, i64* %t1758
  %t1760 = inttoptr i64 %t1759 to i8*
  call i32 @puts(i8* %t1760)
  %t1761 = load double, double* %t548
  %t1762 = bitcast double %t1761 to i64
  %t1763 = alloca i64
  store i64 %t1762, i64* %t1763
  %t1764 = load i64, i64* %t1763
  %t1765 = inttoptr i64 %t1764 to i8*
  %t1766 = call double @Animal_is_quadruped(i8* %t1765)
  %t1767 = fcmp one double %t1766, 0.0
  %t1768 = select i1 %t1767, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t1768)
  %t1770 = ptrtoint i8* getelementptr inbounds ([17 x i8], [17 x i8]* @.slit_1769, i64 0, i64 0) to i64
  %t1771 = bitcast i64 %t1770 to double
  %t1772 = bitcast double %t1771 to i64
  %t1773 = alloca i64
  store i64 %t1772, i64* %t1773
  %t1774 = load i64, i64* %t1773
  %t1775 = inttoptr i64 %t1774 to i8*
  call i32 @puts(i8* %t1775)
  %t1776 = load double, double* %t579
  %t1777 = bitcast double %t1776 to i64
  %t1778 = alloca i64
  store i64 %t1777, i64* %t1778
  %t1779 = load i64, i64* %t1778
  %t1780 = inttoptr i64 %t1779 to i8*
  %t1781 = call double @Animal_is_quadruped(i8* %t1780)
  %t1782 = fcmp one double %t1781, 0.0
  %t1783 = select i1 %t1782, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  call i32 @puts(i8* %t1783)
  %t1784 = call double @separator()
  %t1786 = ptrtoint i8* getelementptr inbounds ([27 x i8], [27 x i8]* @.slit_1785, i64 0, i64 0) to i64
  %t1787 = bitcast i64 %t1786 to double
  %t1788 = bitcast double %t1787 to i64
  %t1789 = alloca i64
  store i64 %t1788, i64* %t1789
  %t1790 = load i64, i64* %t1789
  %t1791 = inttoptr i64 %t1790 to i8*
  call i32 @puts(i8* %t1791)
  call void @__hulk_gc_sweep()
  ret i32 0
}

define double @__lambda_1047(double* %__env, double %x) {
entry:
  %t1048 = alloca double
  store double %x, double* %t1048
  %t1049 = load double, double* %t1048
  %t1050 = load double, double* %t1048
  %t1051 = fmul double %t1049, %t1050
  ret double %t1051
}

define double @__lambda_1097(double* %__env, double %x) {
entry:
  %t1098 = alloca double
  store double %x, double* %t1098
  %t1099 = load double, double* %t1098
  %t1100 = fmul double %t1099, 3.0e0
  ret double %t1100
}

define double @__lambda_1181(double* %__env, double %name) {
entry:
  %t1182 = alloca double
  store double %name, double* %t1182
  %t1184 = ptrtoint i8* getelementptr inbounds ([8 x i8], [8 x i8]* @.slit_1183, i64 0, i64 0) to i64
  %t1185 = bitcast i64 %t1184 to double
  %t1186 = load double, double* %t1182
  %t1188 = bitcast double %t1185 to i64
  %t1189 = alloca i64
  store i64 %t1188, i64* %t1189
  %t1190 = load i64, i64* %t1189
  %t1191 = inttoptr i64 %t1190 to i8*
  %t1192 = bitcast double %t1186 to i64
  %t1193 = alloca i64
  store i64 %t1192, i64* %t1193
  %t1194 = load i64, i64* %t1193
  %t1195 = inttoptr i64 %t1194 to i8*
  %t1196 = call i64 @strlen(i8* %t1191)
  %t1197 = call i64 @strlen(i8* %t1195)
  %t1198 = add i64 %t1196, %t1197
  %t1199 = add i64 %t1198, 1
  %t1200 = call i8* @malloc(i64 %t1199)
  call void @__hulk_gc_track(i8* %t1200)
  call i8* @strcpy(i8* %t1200, i8* %t1191)
  call i8* @strcat(i8* %t1200, i8* %t1195)
  %t1201 = ptrtoint i8* %t1200 to i64
  %t1187 = bitcast i64 %t1201 to double
  %t1203 = ptrtoint i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.slit_1202, i64 0, i64 0) to i64
  %t1204 = bitcast i64 %t1203 to double
  %t1206 = bitcast double %t1187 to i64
  %t1207 = alloca i64
  store i64 %t1206, i64* %t1207
  %t1208 = load i64, i64* %t1207
  %t1209 = inttoptr i64 %t1208 to i8*
  %t1210 = bitcast double %t1204 to i64
  %t1211 = alloca i64
  store i64 %t1210, i64* %t1211
  %t1212 = load i64, i64* %t1211
  %t1213 = inttoptr i64 %t1212 to i8*
  %t1214 = call i64 @strlen(i8* %t1209)
  %t1215 = call i64 @strlen(i8* %t1213)
  %t1216 = add i64 %t1214, %t1215
  %t1217 = add i64 %t1216, 1
  %t1218 = call i8* @malloc(i64 %t1217)
  call void @__hulk_gc_track(i8* %t1218)
  call i8* @strcpy(i8* %t1218, i8* %t1209)
  call i8* @strcat(i8* %t1218, i8* %t1213)
  %t1219 = ptrtoint i8* %t1218 to i64
  %t1205 = bitcast i64 %t1219 to double
  ret double %t1205
}

