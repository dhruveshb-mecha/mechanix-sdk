# BlueZ D-Bus Proxies

This directory contains Rust code for interacting with the BlueZ Bluetooth daemon over D-Bus, utilizing the `zbus` crate.

These files are generated from static D-Bus XML introspection files to keep the module tree clean and separated.

## Architecture

To ensure reproducible builds regardless of the host machine's physical Bluetooth hardware, we use **Vendored XML Introspection**:

1. **The Source of Truth (`/xml`):** The raw D-Bus XML interface definitions are dumped from a live system and stored in the root `xml/` directory.
2. **Automatic Generation & Splitting:** We use the `zbus-xmlgen` CLI tool to parse the XML. When run without an output flag, it automatically splits every interface it finds into its own distinct `.rs` file.

## How to Update or Add Proxies

If the upstream BlueZ API changes, or if you need to expose a new interface, follow this workflow:

### Step 1: Dump the Live XML
You must dump the updated XML from a live system using `busctl`. 

**1. Root Interfaces (`AgentManager1`, `ProfileManager1`)**
These live at the absolute root of the BlueZ service tree.
```bash
busctl introspect org.bluez /org/bluez --xml > xml/bluez-root.xml
```

**2. Adapter Interfaces (`Adapter1`)**
These manage the physical Bluetooth controller. Replace `hci0` with your active adapter if different.
```bash
busctl introspect org.bluez /org/bluez/hci0 --xml > xml/bluez-adapter.xml
```

**3. Device Interfaces (`Device1`, `Battery1`, `MediaControl1`)**
These live on a specific connected device. 
*⚠️ **NOTE:** To capture `Battery1` or `MediaControl1`, you MUST be connected to a device that supports them (like a wireless headset) when running this command.*
```bash
# 1. Find your currently connected device path
busctl tree org.bluez

# 2. Introspect that specific path (replace the dev_ path below with yours)
busctl introspect org.bluez /org/bluez/hci1/dev_75_FB_2E_CC_F5_D9 --xml > xml/bluez-device.xml
```

### Step 2: Generate the Rust Code
Use `zbus-xmlgen` to convert the static XML files into Rust code. Because it outputs the files into your current working directory, navigate into `src/dbus/` first.

```bash
# 1. Move into the target directory
cd src/dbus

# 2. Generate the files (this will automatically create agent_manager1.rs, device1.rs, etc.)
zbus-xmlgen file ../../xml/bluez-root.xml
zbus-xmlgen file ../../xml/bluez-adapter.xml
zbus-xmlgen file ../../xml/bluez-device.xml
```

### Step 3: Fix `assume_defaults` (The zbus Gotcha) ⚠️
By default, `zbus-xmlgen` adds `assume_defaults = true` to the generated proxy macros. 

**You MUST manually change this in every generated file.** If you leave it as `assume_defaults = true`, your code will crash at runtime with `Error: DBus(FDO(ServiceUnknown("The name is not activatable")))`.

This happens because `zbus` guesses that the D-Bus service name matches the interface name (e.g., `org.bluez.Adapter1`), but the actual BlueZ daemon service is simply `org.bluez`.

Open the newly generated `.rs` files and change the macro:

**❌ WRONG (Generated):**
```rust
#[proxy(interface = "org.bluez.Adapter1", assume_defaults = true)]
```

**✅ CORRECT (Manual Fix):**
```rust
#[proxy(interface = "org.bluez.Adapter1", default_service = "org.bluez")]
```
*(Note: For root interfaces like `AgentManager1`, you should also add `default_path = "/org/bluez"`).*

### Step 4: Format and Finish
Finally, format the updated code and return to the project root.

```bash
cargo fmt
cd ../../
```
