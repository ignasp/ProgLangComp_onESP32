#ifndef A_BENCHER_h
#define A_BENCHER_h

#include <Arduino.h>

typedef struct 
{
    uint32_t tStart;
    uint32_t cStart;
	bool started;
	uint32_t tDuration;
	uint32_t cDuration;
} BenchTimer;

void benchTimer_start(BenchTimer * self);
void benchTimer_stop(BenchTimer * self);

typedef enum {
    Match,
    NoMatch,
    NoSTD,
    NotRun,
}RezVerifcation;
const char * printVerification(RezVerifcation);

typedef enum {
    Readable,
	Matlab,
	CSV,
}PrintType;

void bench_Print_header(PrintType ptype);

typedef RezVerifcation (*RunFp)(int32_t, BenchTimer *);
typedef struct {
	const char *    TestName;
	int32_t *       TestLengths;
	uint32_t        Niterations;
	PrintType       Ptype;
	RunFp           RunFn;
}Tester;

void bench_Run(Tester * self);

#endif