---
description: >-
  Warp supports settings for Window size, opacity(transparency), and blurring
  effects. This can help with setting up specific terminal layouts and visual
  preferences.
---

# Size, Opacity, & Blurring

## How to use it

### Window Size

To access size settings, go to `Settings > Appearance > Window`.

* Enable "Open new windows with custom size", Then configure your preferred columns and rows.

{% hint style="info" %}
If [Session Restoration](../sessions/session-restoration) is enabled, Warp will restore the size of the last window closed when you quit the app. Either make sure the custom-sized window is the last one closed, or disable Session Restoration to ensure Warp launches with the custom-sized window.
{% endhint %}

### Window Opacity

To access it, go to `Settings > Appearance > Themes`

* The slider supports setting the opacity value between `1` and `100` where `100` is completely opaque or solid.

### Window Blurring

After decreasing Opacity (moving the slider to a value less than `100`), you can also blur the background.

* On MacOS, this is done using the blur slider. Increasing the slider increases the blur radius that's applied to the background image.
* On Windows, this is done by toggling the Acrylic background texture on or off.

{% hint style="warning" %}
On macOS, large blur radiuses may affect performance, especially on Retina displays.

On Linux, window blurring is not supported.

On Windows, some graphics drivers may not support rendering transparent or translucent windows. See below for troubleshooting tips.
{% endhint %}

## How it works

<figure><img src="https://2297236823-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2F-MbqIgTw17KQvq_DQuRr%2Fuploads%2Fgit-blob-79a7d9b64b98fad12b103e20f755d20d59d1f88c%2Fwindow_size_demo.gif?alt=media" alt="Window Size Demo"><figcaption><p>Window Size Demo</p></figcaption></figure>

{% embed url="https://www.loom.com/share/22c9ef25392e4a5e80f9e01394c84dc4?hide_owner=true&hide_share=true&hide_title=true&hideEmbedTopBar=true" %}
Window Opacity and Blurring Demo
{% endembed %}

### Troubleshooting transparency on Windows

{% hint style="info" %}
At the moment, window opacity in Warp on Windows **does not work** in the following circumstances:

* When using DirectX 12 as the rendering backend
* When using any rendering backend with an Nvidia GPU when "Auto" or "Prefer layered" is selected as the value for "Vulkan/OpenGL present method" in NVIDIA Control Panel > Manage 3D Settings
{% endhint %}

Some graphics drivers and rendering backends may not support rendering transparent windows.

You can select the Vulkan or OpenGL graphics backend to render new Warp windows in the Settings menu, under `Features` > `System` > `Preferred graphics backend`.

You can also opt to render new Warp windows with an integrated GPU, under `Features` > `System` > `Prefer rendering new windows with integrated GPU (low power)`.