#include <sys/sdt.h>
#include "tide-trace.h"

void startroute(char* method, char* path, int id, char* headers)   
{
    DTRACE_PROBE4(tide, startroute, method, path, id, headers);
}

void endroute(char* method, char* path, int id, int status, char* headers)
{
    DTRACE_PROBE5(tide, endroute, method, path, id, status, headers);
}

void fire(char* tag, char* data) {
    DTRACE_PROBE2(tide, startroute, tag, data);
}
