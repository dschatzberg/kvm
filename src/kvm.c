// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#include <linux/kvm.h>
#include <sys/ioctl.h>

int kvm_get_api_version(int fd) { return ioctl(fd, KVM_GET_API_VERSION, 0); }

int kvm_create_vm(int fd, int flags) { return ioctl(fd, KVM_CREATE_VM, flags); }

int kvm_check_extension(int fd, int extension) {
  return ioctl(fd, KVM_CHECK_EXTENSION, extension);
}

int kvm_get_vcpu_mmap_size(int fd) {
  return ioctl(fd, KVM_GET_VCPU_MMAP_SIZE, 0);
}

int kvm_get_supported_cpuid(int fd, struct kvm_cpuid2 *cpuid) {
  return ioctl(fd, KVM_GET_SUPPORTED_CPUID, cpuid);
}

int kvm_create_vcpu(int fd, int vcpu_id) {
  return ioctl(fd, KVM_CREATE_VCPU, vcpu_id);
}

int kvm_set_user_memory_region(
    int fd, const struct kvm_userspace_memory_region *region) {
  return ioctl(fd, KVM_SET_USER_MEMORY_REGION, region);
}

int kvm_run(int fd) { return ioctl(fd, KVM_RUN, 0); }

int kvm_get_regs(int fd, struct kvm_regs *regs) {
  return ioctl(fd, KVM_GET_REGS, regs);
}

int kvm_set_regs(int fd, const struct kvm_regs *regs) {
  return ioctl(fd, KVM_SET_REGS, regs);
}

int kvm_get_sregs(int fd, struct kvm_sregs *sregs) {
  return ioctl(fd, KVM_GET_SREGS, sregs);
}

int kvm_set_sregs(int fd, const struct kvm_sregs *sregs) {
  return ioctl(fd, KVM_SET_SREGS, sregs);
}

int kvm_set_cpuid2(int fd, const struct kvm_cpuid2 *cpuid) {
  return ioctl(fd, KVM_SET_CPUID2, cpuid);
}
