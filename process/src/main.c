#include <errno.h>
#include <libproc.h>
#include <mach/mach.h>
#include <stdlib.h>
#include <string.h>
#include <sys/sysctl.h>

typedef struct ProcessInfo {
  pid_t pid;
  char name[256];
} ProcessInfo;

void get_process_name_by_pid(pid_t pid, char *buffer) {
  char pathBuf[PROC_PIDPATHINFO_MAXSIZE];
  proc_pidpath(pid, pathBuf, sizeof(pathBuf));
  size_t pos = strlen(pathBuf);
  while (pathBuf[pos] != '/' != 0) {
    pos--;
  }
  strcpy(buffer, pathBuf + pos + 1);
}

ProcessInfo *get_processes(int *len) {
  int mib[4] = {CTL_KERN, KERN_PROC, KERN_PROC_ALL, 0};
  u_int mib_len = 4;
  int st;
  size_t size;
  sysctl(mib, mib_len, NULL, &size, NULL, 0);

  struct kinfo_proc *process = NULL;
  struct kinfo_proc *new_process = NULL;
  ProcessInfo *process_infos = NULL;

  do {
    size += size / 10;
    new_process = realloc(process, size);
    if (!new_process) {
      if (process) {
        free(process);
      }
      return NULL;
    }
    process = new_process;
    st = sysctl(mib, mib_len, process, &size, NULL, 0);
  } while (st == -1 && errno == ENOMEM);

  int count = 0;
  if (st == 0) {
    if (size % sizeof(struct kinfo_proc) == 0) {
      size_t n_process = size / sizeof(struct kinfo_proc);
      if (n_process) {
        process_infos = malloc(n_process * sizeof(ProcessInfo));
        for (size_t i = n_process; i-- > 0;) {
          pid_t pid = process[i].kp_proc.p_pid;
          char process_name[256];
          get_process_name_by_pid(pid, process_name);
          if (strlen(process_name) > 0) {
            process_infos[count].pid = pid;
            strcpy(process_infos[count].name, process_name);
            ++count;
          }
        }
        free(process);
      }
    }
  }
  *len = count;
  return process_infos;
}

void free_process_infos(ProcessInfo *process_infos) { free(process_infos); }

unsigned int get_mach_task_self() { return mach_task_self(); }
