#ifndef A_UTIL_h
#define A_UTIL_h

#include <Arduino.h>

#define PRINT_HEADER()\
	printf("|%10s|%10s|%10s|%10s|%10s|%10s|%10s|%s\n\r", "Lang", "Test", "Freq(Mhz)", "Iter Nr", "Data N", "Time(us)", "CPU cyc", "Validation");\

#define REPORT_READABLE(test_name, n_runs, n_data, init_func, cleanup_func, print_func, test_func)\
	init_func(n_data); \
	test_func();\
	cleanup_func();\
	for(int i = 0; i < n_runs; i++) {\
		init_func(n_data);\
		int t1 = esp_timer_get_time();\
		uint32_t c1 = xthal_get_ccount(); \
		test_func();\
		uint32_t c2 = xthal_get_ccount(); \
		int t2 = esp_timer_get_time();\
		int t = t2 - t1;\
		uint32_t c = c2 - c1;\
		printf("|%10s|%10s|%10d|%10u|%10u|%10u|%10u|", lang_name, test_name, freq, i+1, n_data, t, c);\
        print_func();\
		cleanup_func();\
		printf("\n\r");\
	}\

#define REPORT_FOR_MATLAB(test_name, n_runs, n_data, init_func, cleanup_func, data_printer_func, test_func)\
	init_func(n_data); \
	test_func();\
	cleanup_func();\
    for(int i = 0; i < n_runs; i++) {\
		init_func(n_data);\
		int t1 = esp_timer_get_time();\
		uint32_t c1 = xthal_get_ccount(); \
		test_func();\
		uint32_t c2 = xthal_get_ccount(); \
		int t2 = esp_timer_get_time();\
		int t = t2 - t1;\
		uint32_t c = c2 - c1;\
		/*printf("|%10s|%10s|%10d|%10u|%10u|%10u|%10u|", lang_name, test_name, freq, i+1, n_data, t, c);*/\
        printf("t.%s.%s{%d}{%d}.t = %d;\n\r", test_name, lang_name, n_data, i+1, t);\
        printf("t.%s.%s{%d}{%d}.c = %d;\n\r", test_name, lang_name, n_data, i+1, c);\
        printf("t.%s.%s{%d}{%d}.freq = %d;\n\r", test_name, lang_name, n_data, i+1, freq);\
        printf("t.%s.%s{%d}{%d}.rez = ", test_name, lang_name, n_data, i+1);\
        data_printer_func();\
		cleanup_func();\
		printf("\n\r");\
	}\

#define PRINT_MATLAB_D(data, data_len, line_len)\
    printf("[ ");\
    int j = 0;\
    int i = 0;\
    for(; i < data_len-1; i++) {\
        printf("%d, ", data[i]);\
        j++;\
        if(j == line_len){\
            printf("...\r\n");\
            j = 0; }}\
    printf("%d ];\r\n", data[i]);\

    #endif