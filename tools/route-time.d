#!/usr/sbin/dtrace -s
#pragma D option quiet

tide*:::startroute
{
   track[arg2] = timestamp;
}

tide*:::endroute
/track[arg2]/
{
   printf("method: %s path: %s id: %d status: %d headers:%s\n", copyinstr(arg0), copyinstr(arg1), arg2, arg3, copyinstr(arg4));
   printf("Request time %d ns\n", (timestamp - track[arg2]));
   track[arg2] = 0;
}
