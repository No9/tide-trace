#!/usr/sbin/dtrace -s
#pragma D option quiet

tideserver*:::startroute
{
   track[arg2] = timestamp;
}

tideserver*:::endroute
/track[arg2]/
{
   printf("method: %s path: %s id: %d status: %d headers:%s\n", str(arg0), str(arg1), arg2, arg3, str(arg4));
   printf("Request time %d ns\n", (timestamp - track[arg2]));
   track[arg2] = 0;
}
