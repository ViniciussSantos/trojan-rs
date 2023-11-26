# -*- mode: ruby -*-
# vi: set ft=ruby :

# All Vagrant configuration is done below. The "2" in Vagrant.configure
# configures the configuration version (we support older styles for
# backwards compatibility). Please don't change it unless you know what
# you're doing.
Vagrant.configure("2") do |config|
config.vm.define "machine01" do |node|
    node.vm.box               = "generic/ubuntu2004"
    node.vm.box_check_update  = false
    node.vm.hostname          = "machine01"
    node.vm.network "private_network", ip: "192.168.60.21", name: "vboxnet0"
    node.vm.provider :virtualbox do |v|
      v.name    = "machine01"
      v.memory  = 2048
      v.cpus    = 2
    end
    node.vm.provider :libvirt do |v|
      v.memory  = 2048
      v.nested  = true
      v.cpus    = 2
    end
    node.vm.provision "file", source: "target/debug/agent", destination: "/home/vagrant/agent"
  end
config.vm.define "machine02" do |node|
    node.vm.box               = "generic/ubuntu2004"
    node.vm.box_check_update  = false
    node.vm.hostname          = "machine02"
    node.vm.network "private_network", ip: "192.168.60.22", name: "vboxnet0"
    node.vm.provider :virtualbox do |v|
      v.name    = "machine02"
      v.memory  = 2048
      v.cpus    = 2
    end
    node.vm.provider :libvirt do |v|
      v.memory  = 2048
      v.nested  = true
      v.cpus    = 2
    end
    node.vm.provision "file", source: "target/debug/server", destination: "/home/vagrant/server"
    node.vm.provision "file", source: ".env", destination: "/home/vagrant/.env"
    node.vm.provision "shell", path: "prepare.sh"
  end
end
