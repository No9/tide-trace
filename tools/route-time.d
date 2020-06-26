#!/usr/sbin/dtrace -s
#pragma D option quiet

tideserver*:::startroute
{
   track[arg2] = timestamp;
}

tideserver*:::endroute
/track[arg2]/
{
   @[copyinstr(arg1)] = quantize((timestamp - track[arg2]) / 1000000);
   track[arg2] = 0;
}
