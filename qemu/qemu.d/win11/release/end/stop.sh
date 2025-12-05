#!/bin/bash
set -x

sleep 5

# Attach GPU devices to host
# Use your GPU and HDMI Audio PCI host device
virsh nodedev-reattach pci_0000_01_00_0
virsh nodedev-reattach pci_0000_01_00_1

# Unload vfio module
modprobe -r vfio_pci
modprobe -r vfio
modprobe -r vfio_iommu_type1
modprobe -r vfio_virqfd

# Load AMD kernel module
#modprobe amdgpu

# Rebind framebuffer to host
echo "efi-framebuffer.0" > /sys/bus/platform/drivers/efi-framebuffer/bind

# Load NVIDIA kernel modules
modprobe nvidia
modprobe nvidia_modeset
modprobe nvidia_uvm
modprobe nvidia_drm
modprobe drm_kms_helper
modprobe i2c_nvidia_gpu
modprobe drm

# Bind VTconsoles: might not be needed
echo 1 > /sys/class/vtconsole/vtcon0/bind
echo 1 > /sys/class/vtconsole/vtcon1/bind

sleep 5

# Restart Display Manager
systemctl start sddm
