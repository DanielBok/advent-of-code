use std::collections::{HashMap, HashSet, VecDeque};

const PUZZLE_INPUT: &str = "CYJ)BQR
KX8)YWJ
45Z)R38
N95)Z8Z
KG2)1MS
GM6)DB2
TR1)HH5
WTY)P2L
K8J)SSK
VD6)RZ3
NPK)V1G
1T5)49D
TH3)MGC
23D)F12
YRS)MNC
QHP)MJV
GRR)39N
6BV)D8T
3Y9)XNY
J2H)FXH
SDC)172
LCB)1D1
G2G)KM3
5SS)2QY
QRJ)64S
PXZ)SHW
MNY)Q1X
6KN)XJD
HVK)M5Y
X5R)H48
NSB)NK4
DPM)927
S22)B1L
5MN)SF4
RPP)BPC
W58)1Y6
XSN)FT9
3TM)WY7
JQQ)LZJ
VNH)K9T
KG8)HG4
2TJ)DKJ
MHG)G38
1QQ)HJC
92Y)RZW
5M1)QKD
YKP)NSR
3VN)3CM
3FL)JS6
TJV)YFT
6DD)CWH
GKV)8PS
5VG)4KC
1ZM)6P8
RZ3)GGJ
NM1)1DV
SYQ)7XV
T1H)NV8
9Q1)B3P
GY8)SWJ
X6R)2VM
TGP)TDS
HGB)WZV
VFF)XB8
2BJ)3V6
KVX)XS5
NH9)GBP
2BX)PHX
243)ZMN
H4X)KYQ
YXF)DLN
FTJ)JLW
VKH)4JN
7D2)F16
41W)FZS
FJ6)RJS
XN6)53W
SH6)NKL
HT5)XY2
FW9)1LT
LJF)TR1
6CX)WKY
XCR)69Z
Y5H)83F
Y19)G4D
H27)YD6
XQ4)S4R
QM7)F4R
XWM)WH3
XFV)ZF5
6C2)P6M
YXW)GRJ
HGZ)J16
F4R)JNH
J8F)5K4
DDQ)QR2
FZ8)N7L
7W8)RBR
WPM)PX2
KDD)H3H
LPJ)7KS
YP9)KNL
MPZ)C54
TB2)F2Q
FLY)4YY
ZX3)YBR
FW6)HG2
SMQ)J4H
XB8)FDK
39T)5GB
WF6)SMC
QXK)VBV
MM7)MPZ
WVF)BQ2
PKM)DTF
TWZ)158
K7X)64D
3WJ)7VQ
24C)BJF
Z5Y)Q5H
1KB)NSC
ZRM)X86
HJ5)N4C
PTV)RCH
W3J)4SJ
H8Y)QG4
HFT)Y7D
J6C)QKZ
6L4)CBY
WT6)NM2
XLT)G3K
MW9)HPS
CFF)VFF
3BM)J6C
VPN)LHH
FHW)WS8
DQ9)K67
DN7)8LM
DVF)6RM
SY8)ZQG
Y5V)QYH
XVT)3T9
H5C)XP7
MHR)PCY
22H)N93
9QV)511
495)XR7
TML)7XT
KYZ)1RY
Z4Q)WDP
QJ2)L2P
V2H)5RR
JC6)2PB
4YS)G42
DGD)KBW
DR3)WTD
2MK)FGQ
D9L)2QT
BFN)RK1
QGD)WS4
PS8)NXZ
K4L)712
SLC)HB8
HJN)B4Z
R2Z)Z2Y
MY6)LNM
VQ3)L1W
RBH)VKH
WF2)3RP
D9R)Y8M
7R3)WWK
4PZ)DN6
JDF)XJB
G4D)VPW
XT6)J31
G4D)JJT
BRZ)1X6
C54)Y4F
BTC)ZM9
2HL)1KB
1KB)QJ2
JTV)J2J
Q6B)V5T
LDT)KPD
P3H)4V3
4RB)GKV
42C)RDX
B4G)HT8
47S)6Q4
XZ1)MNY
Q3X)67Q
65Y)ZYM
2NG)FDD
QKZ)LPJ
158)4SP
8VD)CYJ
9DX)HTQ
MZY)WZ2
4SP)FVY
81D)JX3
8GM)LYW
BLD)CBP
G3C)1HY
65S)91G
XXH)GCT
HCR)WZC
FSD)YJ9
14N)MBW
DVB)BWS
74Z)B5C
YB4)LBF
FJS)S4L
LJP)YCQ
DQ4)C34
H17)LT3
81B)VBM
DPG)8BN
N9R)W6P
2FH)65S
8Q2)J8D
FTQ)55P
FPD)DSP
1TX)4W9
HXL)8NM
CG7)G2G
X52)6JY
4S7)D9R
XQ4)TH3
NZV)XV2
LZH)QV6
V8N)FFG
JGC)C1X
1GX)78B
RCH)6C2
PKT)88J
TF5)M3T
W5J)WVF
QMM)B13
THR)RL7
396)GRR
XVV)H8Y
MBW)39K
DQ1)SH6
XVF)KYZ
26W)8JM
YWF)ZH1
W61)D54
NCL)PCD
FSG)BLM
GFX)RYY
SJ7)X7L
7D6)5QP
PB1)87B
2PJ)MQG
XG9)NRR
WT6)JGC
J99)SHD
143)GQL
ZFK)PQ4
PR8)ZFK
CV3)TKK
6P3)R3F
XS5)W58
QBN)NFY
6Q4)4TW
Q4F)3CV
3L7)M6P
ZGX)5VT
MQG)2Y2
3VM)HVY
BX4)HLB
FZS)S2B
XDG)PWD
FRZ)4T5
TC8)97K
G3K)X4P
8TW)6KN
B71)S9D
CDH)Y5V
V11)Y4S
JWW)RGD
F16)TF5
S1W)LTT
J99)BJ7
YBV)QPT
5MJ)YXF
4YY)YM1
J2J)WD8
MJV)9DX
MJN)HZN
BBY)JWC
1W7)G9L
3N3)SBH
NFB)4B8
H83)S1P
TG3)G6T
25R)B7G
PCZ)Q56
BGX)N8K
687)HFT
ZZ3)RZF
Z2T)BHW
LFB)2NG
SJW)1GX
VMZ)CDH
YM1)WDR
PCD)Y2X
C1Q)69F
8L9)9CL
DP1)FFC
Y4L)DF1
Q1Z)WFT
NP8)4YS
H1R)DDJ
8PS)PZL
G1Y)LWD
XV2)BWN
XVY)956
8XR)ZPR
W8L)L7G
RYY)MSD
YWB)P3P
RCV)9X3
WHV)23D
CH7)J6Z
NZ9)58Y
ZML)YOU
SZC)Z9T
XXH)NSX
WPM)C9L
5NW)P3H
HP5)FR7
LSJ)XZG
93S)THV
XF7)J2Z
SFV)9WS
5BY)2N2
GR5)MJD
RVR)ZCN
QV6)CKS
GFQ)3D6
4NQ)8FY
VFT)W2F
2LZ)939
9D6)FJ6
NCY)37Q
J2P)HJN
3GF)H1R
NPZ)J99
VGJ)M8W
D92)B71
K6F)XXH
PQ9)83R
1Y6)NZN
CDD)RQL
WF6)DFD
K97)PTV
4BR)Q6B
MBJ)PZG
DG2)ZN5
3YJ)KG8
7XT)PXZ
J16)SKX
SC7)DQ4
GBP)QBG
SQM)VQ9
WST)X78
ZSQ)GS7
J4V)DPG
7KS)J1P
PD6)2PJ
S9D)FGG
5XH)XPC
JV2)JH5
SMC)VZT
P89)5N3
FR5)4W8
WK4)X3C
T68)DDQ
CYG)D52
P6M)MKR
W7P)8V8
L9K)LFZ
QQM)M5C
VJL)7Q8
NV8)39T
XPH)DR4
Q5H)PTG
2HD)4H7
KPD)D1H
CHT)Q1Z
TXW)CFF
STC)MHY
XZG)3YJ
7Q8)Y59
FVY)FTQ
CX5)MW9
VTK)FK2
Y2X)VYS
TG9)2HD
32Y)2FH
J7W)MX8
LX7)5BY
HTQ)SQJ
B4Z)NCT
MNX)DR3
MKR)BPZ
HPK)SGL
LZJ)5SC
1RY)DQH
3WJ)61L
4GR)M6H
G42)PMS
NJ8)JJS
D1M)SSV
NM1)CPT
DGF)1QQ
NWL)WF2
F3Z)KTZ
6MF)ZN7
DKL)HLR
ZK9)2PC
JWW)NLV
7Q8)X52
S4R)JZJ
BJ7)MZY
F3H)V37
RD5)TV7
TXK)722
7QV)MTP
DQH)82P
7SR)SFM
CBV)XCY
MWP)WT6
MSK)NMM
5JN)13Z
9NZ)3Z4
M96)N6T
WVF)512
K67)4S7
FKD)H83
R58)5XH
SSK)WWX
5N3)FT5
T64)N8C
53L)X3Y
NV8)Y66
PPD)4JV
RL7)HJ5
35D)8GM
85S)WRW
BWN)MQ5
S1P)PNS
X3C)JKZ
15T)65P
TX7)N5N
MHY)5PB
SQQ)Y19
5NH)3ZG
S4M)GZF
512)87J
NB7)5HT
NK8)3F4
CWZ)3VN
ZW7)T5B
FR7)X48
7P1)VQ3
5WN)HDH
FXC)TG8
M6Q)SQM
GTZ)3GM
YZK)5MJ
Z9P)Y16
FGG)36F
23M)FK1
YXF)FH9
ZM9)ZRG
RL6)ZML
6Q8)TBD
2Y2)PH5
HRD)Q9S
P5M)6P3
QPD)MMC
YHH)W3J
43B)3L5
QPH)VD6
P4S)SX3
VGZ)4NQ
39N)HDR
P2T)2HL
Z4N)TNT
9TQ)HML
5RR)VX5
L9K)YTT
3M2)HRD
M6H)K8H
W5M)B4G
4LY)2X8
S4N)PD7
84S)DHK
3L5)58T
N1Q)BD2
MD5)DKL
LYW)BDM
88J)MFL
B8Y)MBT
H9X)8DQ
2XW)DJW
T7G)FQL
BD2)LK3
SJ5)8LS
64S)WC7
S1Z)9C9
DSP)P89
XPC)MSK
4KM)LHS
JRX)KVX
4WY)9SB
232)422
MZV)X4Q
WKK)LRC
TNV)3M2
DJW)J7S
R38)RY9
2PC)2CD
WDR)NRF
FY3)PK2
TXW)VL1
NH6)MG9
M7D)XBJ
WXF)Y8Z
XVF)BM7
YJ5)396
KT2)C1C
SGH)ZGB
67Q)PR8
VV9)G6F
V1Y)YYP
722)FZG
R18)HXX
LFH)PHQ
HT8)RQG
1PM)V8N
4SJ)8KT
LRC)S9C
6W7)HP5
GXR)TB2
L75)N8X
X7L)BYF
QHG)Z2T
DSM)425
C93)C2K
GT6)LPQ
6RG)TQR
3ZG)ZRM
JSH)XQ4
L3H)WW4
GZB)5DG
M22)23M
FL6)V1Y
LJF)92R
7R1)H17
1PM)B8Y
J1P)K7X
J7S)BGX
372)DQ9
BGY)42X
9X3)L2X
BVN)S1W
61W)224
SGL)KT2
XR7)M9G
DR4)H9G
4PK)1XL
DN1)2XV
FZ3)62V
X86)LQP
X9R)BJ8
VST)PYV
4V3)1V4
2RP)5M1
WFT)FW6
V6V)4SV
4SV)947
ZN7)XWM
1BB)SDC
WFT)J4V
BPC)2SC
T6X)521
YTT)SJW
6KP)P9L
NXQ)X8J
WCS)XVV
YPD)JQQ
42G)85T
5RM)YP9
MSB)GZB
DD4)YP2
N29)622
FZM)8L9
2RX)HXT
DSF)746
LWN)JMR
91L)WLQ
425)TG9
DFJ)DSK
TWG)CC3
JR3)XPH
NLV)K4L
S4L)XQK
JVG)J2M
DD6)TG3
GTD)8PL
947)3LK
WM4)NZZ
WLQ)N9R
GZ4)DQ1
37B)448
275)43B
Y6V)MFH
277)SLC
NHL)877
9LP)WDX
G6J)RWG
NVK)LBT
R9R)6NG
MTT)QJY
HJW)TBY
T3V)JHK
363)6D2
JX3)3L7
5Q4)9DZ
F7N)DRW
PWD)T1H
P74)8XV
S2P)FYV
X48)SS3
DFS)CT5
SF4)PD6
NZZ)G8S
DHK)Z5T
MY9)DQT
4NJ)6ZW
KN5)FFD
NWL)NZR
CHQ)JY7
YJZ)GT2
9L9)N95
NPK)JFV
SG8)Z7J
92R)X9R
JNS)H27
XGN)JDF
DFD)MBJ
Q4N)6DQ
622)MYH
T9K)MD5
HYX)26R
2K8)GTD
S9T)G37
MBJ)33K
QGX)VNH
NZR)781
3D6)58B
76C)MMR
COM)Q2S
Z2T)VGZ
5SC)XGY
1LW)42C
PPD)SLN
4TW)WPN
XYB)R2Z
Y4S)KR7
PHX)VPN
TS8)7R1
ZCN)V66
73H)NB7
X78)DPM
41D)MZV
TD9)5TW
DF1)JKN
8V6)5QS
8NX)2D9
4ZX)5WJ
MBW)SK9
LYL)ZJL
WHQ)363
H1H)SN8
KFV)XR9
521)6KP
Y8M)KRJ
C3G)HPK
HXX)5RM
HP1)D7Q
WY7)5F2
8YV)TXM
QPT)HT5
ZYM)Z2K
C1C)THZ
NXQ)1BB
ZGK)N13
6NG)RTS
S3G)V4X
KSX)KPM
CPG)NH6
3Z4)ZYK
D7M)TYR
BHW)LCB
Q1X)BKY
HJC)Y3C
WX7)QM7
ZJL)Q5M
JRW)5HZ
RDX)JPW
VL8)YRS
W2F)41D
8JM)YYL
RLT)9Y1
M6Y)1KH
6S6)3G5
DLZ)T64
QDH)232
ZG7)35D
WWG)DN8
476)SGH
R51)B96
6QY)LYQ
3BZ)DJD
33K)LFH
N8X)3G3
SX3)1PC
69F)WF6
9YQ)XWG
N2N)RFD
QVN)BWW
N1T)PQ9
T5B)VSH
JPC)VNV
2X8)D9L
B13)YG4
2QT)3YQ
G88)84S
HPS)TS8
5XV)HXL
YG4)KSX
DN6)1KR
JJS)45Z
HZR)2TJ
6TM)M48
XVQ)3YD
N6T)F26
4CT)BZN
36J)3HJ
YKP)36R
FST)LB5
HP2)DGF
RRZ)LSJ
PNS)RWK
SN8)F5N
CM3)DS5
JKC)L5C
N8C)FHW
39T)NQB
DLN)CM9
MG3)K1L
QDH)XVF
KR7)XT6
YD6)3X4
ZK9)WST
NFY)MH6
3RP)78X
9WS)PWZ
WDL)SKR
K7M)9TK
JSH)4PK
D9L)R43
58Y)3DK
KBW)189
8V8)22V
QBG)162
1D1)K9J
PQ4)3RL
BFF)LSK
VMW)BPD
BJF)6CN
BQ2)JTV
BWW)MWP
8YV)TML
LSK)RRZ
WGK)LL7
M6P)XGN
1LT)95V
4B8)14N
23Z)NMR
TDS)72D
712)SQN
CBP)CWZ
172)JVG
PMS)2Q1
XJ4)5ZC
5ZC)3H7
3X4)MHG
2SC)16L
MQ5)LYL
8FY)TTW
LF1)Y1L
FQS)VKL
VGP)687
J8S)KGB
DN1)PLP
5XH)H5B
87B)HZ5
SWJ)LV5
SVM)WXF
YJ9)76C
BM4)P9F
PK2)BFQ
94W)MVC
X3Y)FL6
36R)VYZ
X4P)Q3X
DKJ)2LZ
78X)QPH
DD3)TWZ
PV6)8DF
N4C)9ZM
129)T4X
WPN)8Q2
SKX)265
XXD)G2X
SSV)MM4
RM6)3VM
NRR)JWW
HSW)9ZV
SWK)W5J
WDX)DVM
4NP)WHQ
TNT)J7W
R4T)JDZ
DYS)5P1
QGR)VRM
BZY)MY9
N39)LDT
J4H)865
FK2)TKD
1MS)JSN
DB2)3Y9
NMR)17B
Q9S)WCS
Y5V)S9T
657)MNX
3D6)G3J
CBP)RHW
888)WX7
Y68)847
54J)54B
5NR)XF7
HMY)W4K
MM4)DCG
6RM)7TT
23Z)YT1
VX5)9N6
35D)NJ8
MFL)Y9J
P7Z)Y68
KRS)12V
93B)NHL
MGL)KH7
S16)KR2
NSR)5VG
H3H)7SV
FQT)QVV
2Q1)73H
N25)2VK
JFP)9K7
HMW)K1X
B55)2XW
KBW)ZD5
VV9)7Y1
FRJ)VLS
G76)HMM
TYR)HGZ
2VK)YBV
KFC)6MM
DZM)W6D
LPQ)3P7
M5C)FW9
FNP)JKC
XZ1)LF1
BJ8)RGB
J8D)J4M
X4Q)HCT
4QW)K8J
1GC)8TB
N95)78D
R3F)RL6
7CV)N25
JJT)84V
FFG)GFQ
L3W)MZ7
61L)MD2
7VK)RCV
VSH)RDP
8DF)6TF
5RC)DGD
PX2)HKP
KXT)MLL
V37)G6J
Q5M)VYL
311)NTY
2LM)LWN
QKD)NSV
6TF)QVN
5K4)ZHK
72Z)S62
781)6XT
N1Q)VWZ
6XT)26M
83F)NZ5
7F3)4JB
6V5)KXT
3G5)2BJ
CJ1)H5C
HDH)53Q
9J1)K6F
MNC)9QL
KFY)QKX
GRR)GY8
F12)YFX
CX7)22H
Y16)888
F9F)CYG
BMK)PDQ
SHP)243
7Y1)YLD
N1T)P1H
H5B)QMM
4GR)NZ8
2XB)SFS
8NM)PB1
KQ4)6TM
JNS)XVT
PMC)FXC
V6G)SZC
3CM)N6Y
22W)QQM
947)1ZM
K8H)5NK
D52)9NZ
TCS)MSB
CKS)DG2
YDX)L79
XD6)D3K
265)8TW
26K)BMK
427)L24
GQL)SZS
WS4)7JN
G2X)QCL
N8T)8YK
FFC)T3V
G24)K47
8BN)M2X
YJ5)2R8
7TV)ZV1
9N9)YM4
JDM)TC8
RQL)3GF
3RL)65Y
QG4)TYP
3X4)DVB
GZP)D1R
422)VGP
R43)8JV
3YQ)V2H
HB8)G24
N7L)17Y
KH7)GKH
LXX)37B
78B)LSG
Z9P)2WY
FDK)1PM
GD4)81B
C7D)774
3S8)NX1
GKH)6PS
MD2)XDG
TV7)5NH
PLB)F4W
SQN)68X
GCT)6DD
3V6)Y6L
JLW)CV3
FGQ)HVK
37Q)5NW
KNL)J75
LHM)CG1
FTD)FQT
SGH)LX7
L2X)LXX
5RM)7KC
8TB)1PY
VBV)23Z
RTS)JV2
DN8)LL9
K1X)CM3
WWX)2P7
NZN)G76
TD9)PT8
29L)W52
D1W)YZK
VYL)Z4Q
X3Y)S1Z
VJT)DP8
ZH1)934
CSZ)FTN
739)R7S
NTY)TX7
3YQ)TCS
KRJ)N8T
YRS)9J3
P5F)FGW
JFV)WGK
JPC)X7D
P3H)JFP
RDP)PSM
6T9)5RC
9Y1)GM6
PCY)P2T
V1R)NYK
SHD)TB5
PSM)1L2
491)NM1
ZMW)SHP
TNT)ZGX
HDR)BZY
Z91)4WM
9J1)S3G
NQB)3FD
JMR)L3W
8JV)ZSQ
3FD)7VK
9K7)59S
B1L)CPG
FL3)JDM
R6Y)SG8
8XR)2RY
5PB)N4F
N93)ZKQ
LPV)WB4
HG4)4LY
MFH)PF6
VPW)PF5
PT8)2K8
DCG)DFS
Z7P)TZT
P1H)QHP
G6F)X5R
TBY)WJG
LNM)2FW
Q83)G3C
YCQ)SFV
RZZ)8DL
TQR)36J
82P)ZX3
K4H)275
PSR)BWZ
511)JRX
HLR)VMW
SLN)Y6V
4XZ)8B8
NMM)3PX
MH2)5P5
Y8Z)WPM
GFX)5JN
2R8)JSH
3YD)FRJ
BPZ)HGB
2P7)QGX
B5C)TSK
VSH)XCR
1PC)Z7G
J62)577
HKP)NP8
MLK)SBR
NYK)KLP
5CP)HCR
MSD)Q4N
2TJ)PV6
HZ5)21W
2WY)X6R
6D2)FG2
W6D)5NR
F67)J62
M6Q)9BG
X1M)CNG
JKH)YGG
Z84)77T
LBT)3WJ
XQK)Z7P
1BB)S4M
SBR)7F3
B9M)3FL
42X)W6J
2VM)S2P
RRG)1P2
QRJ)RM6
T2M)4BR
XNY)RD5
53Q)CBV
WKY)GR5
PZG)2D7
RJS)N39
YBK)H9X
FR5)MTT
9TK)HZR
5BT)YHH
D73)XM5
L61)YXW
4NJ)D1M
865)W6W
58K)V6V
BQ2)V2D
LFZ)HMW
YM4)93B
JPW)7TV
FFK)6QY
FZG)5SS
YHH)L75
RHB)9QV
23M)9YQ
W4K)BFF
THZ)M4N
HQD)N2N
BWZ)DFJ
RFD)T3G
SBH)427
PNY)58K
WWK)3S8
48Y)F9F
1XL)FZ8
CPN)YDX
5HT)1PN
8KK)D3S
QR2)VWB
MMC)FY3
M4N)RC5
SK9)K33
GFK)KG2
G6T)GFT
TZJ)XD6
CM9)FV6
K5Y)2M1
M48)LZH
TSG)42G
FSD)66S
SKR)NH9
2YD)WDL
QG4)LQC
2KK)3LX
GS7)T9K
V5T)Q4K
K49)MSC
STW)Q83
5HZ)YBK
WZC)J8S
2PB)SW1
BLC)XFV
3CV)TVJ
MBV)K5X
SFM)C93
SHD)8HP
YT1)TZJ
7KC)QJZ
JD7)9Y9
ZNQ)6XQ
2VQ)MLS
YQ8)DSF
5X9)143
F5N)2VQ
Y3C)2LM
TB8)XW3
XM5)29L
Q6Q)ZDP
6X8)8S3
S4N)MH2
8NT)8KW
5BT)7CD
MZY)LPV
FT9)ZMZ
ZKQ)M6Y
JNH)79Z
JZJ)YR6
KCY)32Y
TVJ)MHR
577)KK6
1L2)M22
8LM)RDM
7TT)CHT
F4W)J2H
KR2)6M6
WC7)HCD
YJ9)ZK9
D3K)F67
81B)6MF
JDZ)8Q7
TYP)GFK
QKX)YWB
75P)GD4
8JG)5X9
16L)4DC
PQW)H4X
CL5)BVN
ZHK)DFB
S9C)NK8
8PL)WF1
YWJ)VTK
6ZS)P5F
KVX)FL3
XY2)3BM
P89)M96
XJD)VMZ
WG1)TY6
WX2)YB4
1DV)J3S
FCN)QH7
5GB)QDH
R58)MGL
GGJ)2Y5
NSX)1GC
KPM)DP1
3H7)YTF
BQM)KDD
MGC)YQ8
2N2)XSN
VYS)SJ5
SW1)LKP
W8P)BGY
LGG)T68
3VC)77C
CN8)K49
HZN)91T
1JK)GXR
HXT)VV9
Y59)9PZ
GFT)S16
ZYK)2B8
Z76)QRJ
HQ3)ZNT
21W)SMQ
SJ7)T7G
5NK)FPD
QJZ)LMF
9ZV)NCY
WTD)JD7
2D9)BX4
67Q)WKK
8DQ)M6Q
B23)4ZX
12V)ZJ2
HCT)2BX
MRH)76S
XW3)75X
85T)JC6
QLX)N9D
DHC)RVR
62C)L6R
ZV1)KWX
X7R)PSR
BDM)FRZ
FFD)RL9
3GM)62C
Q2M)HQ3
49D)PCZ
BJS)529
THV)PMC
RY9)H54
5SS)B55
P9L)BBY
85S)433
NK4)P5V
MJ7)Z91
8JV)KQ4
39K)TWG
LYC)KCY
26M)JR3
PD7)ZG7
H9G)1W7
RD6)1VG
Q34)C3G
J2Z)THR
3B2)CSZ
RGD)SJ7
76S)TSG
XBJ)SQQ
9QL)KN5
LQC)5K1
RDC)Y4L
MLL)P7Z
6WZ)TB8
6L1)GZP
BVD)FLY
C9L)J8F
L3W)N1Q
RBR)MJ7
SJW)QBN
YYP)129
4JN)K5Y
DYL)PDW
FW9)FZM
VL1)R51
N6Y)6RG
ZJ2)54J
M2X)KX8
1V4)WTY
4H7)5Q4
BLM)F3H
75X)7QT
Z18)LJF
M3T)1NW
J6Z)D92
9ZM)PKM
WZ2)92Y
22V)TJV
WMR)WG1
VQ9)74Z
LL9)8W3
9DZ)KFV
D52)J2P
KM3)QZ9
YLD)FCY
8TH)P9T
BZN)GZ4
F2Q)DLZ
Z2Y)V1R
HMM)72Z
QJY)176
FK1)QV5
VKL)Q34
33K)495
17Y)3N3
WC7)GYW
3VS)SWK
VRM)6L4
JMR)Q4F
NNM)CPN
3PX)6Q8
M5Y)XLT
GP5)MCC
MX8)W8P
RHW)B23
Z2K)MM7
FV6)V11
GM6)372
69Z)TNV
L2P)LYC
1KQ)YJZ
847)CN8
GT2)BRZ
6D2)PCG
BYF)477
Y8M)7W8
T4X)FCN
JS6)GP5
RZF)PPM
4JV)RD6
MJZ)W61
XR9)CL5
KTZ)5CP
971)WX2
2WY)8FL
1P2)RDC
NSV)9TQ
PWZ)ZNQ
ZNT)6V5
BLL)NFB
5QP)4Y9
SDC)4WY
NZ5)9P6
ZPR)MLK
77T)B48
GGH)Z18
4PQ)R4T
Y1L)NCL
LT3)K4H
2RY)971
DDJ)Q2K
WNR)S7H
D8N)NBN
P9V)NWX
TPR)MRH
GZF)SY8
D8T)K97
WB4)NXQ
XLX)JRW
Y4F)HMY
162)KRS
JQ3)DYL
6C6)QGR
TY6)ZW7
WD8)LNQ
6JY)NNM
DB2)NZ9
433)QNW
6ZW)YQK
FT5)4T2
6MM)2YD
1X6)TD9
MBT)76G
LLL)W8L
K9T)GT6
54B)S4N
YFT)6CX
N4F)3B2
PX4)3VS
PJW)XJ4
YG4)4PQ
2FW)R1T
VLS)45P
QFL)4CT
TXM)2RX
QVV)98H
78D)6RD
TKK)NJV
8HP)6ZS
P2L)Z9P
XCY)94W
9P6)F7N
9N6)2BM
ZN7)LFB
1BG)6W7
DPM)1TX
S62)5BT
MTP)9L9
Q4N)BLC
N4C)FTJ
VWZ)NVK
1PY)7Y6
MZ7)X24
V1G)7P1
S11)8YV
2D7)TXK
2XV)Z76
YNV)3BZ
P3P)CNT
BPD)P9V
V2D)Y5H
CPT)T8H
MMR)ZQY
NXZ)XG9
HG2)7QV
HZM)1T5
J75)X1M
WTY)PQW
QCL)D1W
FCY)SC7
1F6)M7D
D8T)QHG
B7G)75P
RXH)MG3
XJB)NZV
J3S)CVW
PX4)FR5
5VT)JPC
MLS)BLL
3YD)TGP
ZRG)6T9
9BG)HP1
M7V)53L
SGZ)DN1
26R)FQS
LK3)Z4N
7SV)SVM
N56)WK4
JP1)QXK
4JB)S11
5K1)W7P
WZV)311
939)SAN
X8J)ZZ3
7CD)SYQ
RWG)GJN
QYH)491
L7G)6WZ
53W)JQ3
Q2K)7D6
BQR)YKR
RC5)4QW
XR9)5WN
ZF5)BJS
83R)NSB
SMQ)LLL
DHC)QFL
L79)DZM
FCB)6VY
4W8)PS8
DRW)HLG
JWC)B8N
L1W)9V9
DN8)P4S
YBR)QQD
FXH)D8N
X7D)RLT
ZQY)CHQ
62V)KQZ
W3L)CDD
VYZ)R9R
V66)4PZ
C34)3VC
YQK)1KQ
YGG)8XR
97K)BLD
QV5)WHV
RGB)MBV
RK1)R18
S7H)6X8
Z7J)61W
V89)NT4
B4Z)VST
JC6)L9K
9CL)ZMW
4YS)CX7
SCD)6L1
GYW)24C
N13)7SR
1VG)VS2
Y7D)HJW
G3J)HQD
189)SCD
Y59)V6G
QNW)C1Q
TKD)PNY
K5X)DQJ
MYH)SGZ
JSN)FCB
D7Q)WTF
ZMN)W3L
2FW)T6X
746)C7D
3T9)FKD
98H)2MK
XS8)P74
5F2)K7M
2BM)4Z6
Z9T)WTS
HVY)HN3
6CX)JNS
372)STC
72D)DN7
KK6)8NT
BFQ)G88
PZL)48Y
WH3)TXW
N5N)3TM
LYQ)VGJ
3LK)4NP
45P)Q6Q
H48)RBH
Q34)XXD
8FL)D73
8W3)NPZ
VWZ)H1H
54J)HSW
TB5)5T5
ZPR)M7V
4T2)HYX
JJT)277
QT4)6C6
66S)L4R
H17)YLL
657)7D2
RGK)WWG
5P5)MY5
XSN)JP1
ZML)YNV
8KW)FSG
RK1)4GR
176)5CM
RDM)TPR
ZQG)8VD
4KC)41W
65P)FFK
927)91L
TTW)WM4
HCD)CH7
G37)2NZ
LWD)YWF
B48)RHB
DJD)26K
MG9)85S
5XV)X7R
8MK)8NX
MJD)B9M
9SB)XLX
477)739
FH9)42Y
6RD)LJP
WJG)PJW
XGY)5MN
WTS)FJS
GZF)2RP
3G3)MTR
BV6)KFY
R1T)4RB
64D)FTD
61W)15T
JGQ)HZM
WDP)DYS
68X)SZG
2PB)CJ1
L2P)9J1
Q2S)476
VNV)MJN
5TW)81D
ZGB)N56
HCT)DD4
87B)YKP
6DQ)STW
2B8)N29
JKZ)GTZ
1KQ)4XZ
W8P)6GF
NX1)47S
HKP)DD3
RWK)W5M
2NZ)BM4
DVM)QPD
KTZ)RGK
NJV)4NJ
PK2)S7M
H1R)CG7
1KR)5XV
Z5T)4DD
GRJ)4KM
L5C)XVY
VS2)VFT
TG8)PKT
1NW)HP2
KYQ)YFD
PNS)NQ6
PF6)Z3C
VBM)8KK
84V)ZGK
YWX)BLH
LTT)D7M
MTR)NQX
3Z4)657
91G)JKH
5DG)GFX
5QS)1BG
DTF)RXH
KQZ)DHC
7VQ)RKH
FG2)25R
9PZ)MJZ
7XV)V3G
KFY)DD6
3F4)NWL
XP7)VL8
4WM)BV6
Z7G)WMR
PF5)PPD
F12)1JK
FQL)RRG
W6J)8TH
5P1)PX4
L4R)T2M
934)V89
Y9J)S22
Y4S)YJ5
MSC)BQM
NQ6)7R3
36F)8JG
JH5)SPN
Q4K)VJL
7JN)VJT
PH5)DVF
W6W)MY6
4T5)6S6
P9F)7CV
RL9)YZC
SHW)XN6
HDR)XVQ
3HJ)BVD
BKY)YPD
2CD)BFN
TBD)542
YR6)XS8
Z3C)6BV
8KW)NPK
T8H)1LW
SFS)8MK
V4X)RZZ
ZMZ)FNP
FGW)1GZ
K9J)R58
224)YWX
55P)58W
C1X)WNR
KLP)FSD
LQP)CX5
J4M)XZ1
8GM)GGH
V3G)8V6
G9L)FZ3
YLL)JDW
939)2XB
X24)L61
7Y6)L3H
VPW)QT4
5WJ)LGG
W6D)9N9
58W)KFC
59S)F3Z
8B8)G1Y
LSG)PLB
5CM)93S
M9G)9Q1
SZS)FST
LHH)QGD
SPN)9LP
J31)RPP
8KT)Z5Y
ZN5)XYB
2M1)Z84
G38)R6Y
K33)26W
4W9)9D6
W6P)BTC
VWB)Q2M
NSC)22W
7CV)JGQ
PYV)N1T
PPM)QLX
WRW)1F6
6P8)P5M
S7M)D4P
NCT)2KK
ZD5)LHM
WK4)DSM";

pub fn solve_a() {
    let orbits = get_orbits(PUZZLE_INPUT, true);
    let mut queue = VecDeque::from([("COM", 0)]);
    let mut total = 0;

    while queue.len() > 0 {
        let (from, n_orbits) = queue.pop_front().unwrap();
        total += n_orbits;

        if orbits.contains_key(from) {
            for &next in orbits.get(from).unwrap() {
                queue.push_back((next, n_orbits + 1));
            }
        }
    }

    assert_eq!(total, 292387);
    println!("Solution A: {}", total);
}


pub fn solve_b() {
    let orbits = get_orbits(PUZZLE_INPUT, false);

    let mut queue = VecDeque::from([("YOU", 0)]);
    let mut seen: HashSet<&str> = HashSet::from(["YOU"]);

    while queue.len() > 0 {
        let (from, n_steps) = queue.pop_front().unwrap();

        if from == "SAN" {
            let ans = n_steps - 2;
            assert_eq!(ans, 433);
            println!("Solution B: {}", ans);
            return;
        }

        let next_orbits = if orbits.contains_key(from) {
            orbits.get(from).unwrap()
        } else {
            continue;
        };

        for &next in next_orbits {
            if !seen.contains(next) {
                queue.push_back((next, n_steps + 1));
                seen.insert(next);
            }
        }
    }
}

fn get_orbits(orbits: &str, directed: bool) -> HashMap<&str, Vec<&str>> {
    let mut neighbors: HashMap<&str, Vec<&str>> = HashMap::new();

    for pair in orbits.split("\n") {
        let (main, orbiting_element) = pair.split_once(')').unwrap();

        neighbors.entry(main)
            .and_modify(|vec| vec.push(orbiting_element))
            .or_insert(vec![orbiting_element]);

        if !directed {
            neighbors.entry(orbiting_element)
                .and_modify(|vec| vec.push(main))
                .or_insert(vec![main]);
        }
    }

    neighbors
}