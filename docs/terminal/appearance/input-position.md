---
description: >-
  Warp gives you the ability to configure the position of your input, which
  includes both the prompt and the command line.
---

# Input Position

You can select from three different input positions, which each have different modes of behavior for the flow of input/output Blocks.

<table><thead><tr><th>Input position</th><th>Behavior</th><th data-hidden></th></tr></thead><tbody><tr><td>Start at the top (Classic mode)</td><td>When you select “start at the top,” the prompt with input will initiate at the top of the view and move down in the view as you enter commands. Blocks of input/output will stack above the prompt and command input. You can scroll up or navigate up to visit past commands. You can enter <code>CTRL-L</code> or the <code>clear</code> command at any time to return the input to the top of the screen while still maintaining your scroll history.</td><td></td></tr><tr><td>Pin to the top (Reverse mode)</td><td>When you select “pin to the top,” the prompt with input will display pinned to the top of your terminal view. Blocks of grouped input/output will flow down the view in reverse order with your latest results at the top. You can scroll down or navigate down to visit past commands. For long-running commands, you can also click "Lock scrolling at bottom of block" to continue to follow the stdout.</td><td></td></tr><tr><td>Pin to the bottom (Warp mode)</td><td>Warp mode starts with input pinned to the bottom of your terminal view. Blocks of grouped input/output flow up and out of view. You can scroll up or navigate up to visit past commands.</td><td></td></tr></tbody></table>

## How to access it

* You can configure your input position by navigating to `Settings > Appearance > Input`.
* You can also choose and set modes from the [Command Palette](../command-palette).

{% hint style="info" %}
Changes to the Input position take place immediately and apply to all open panes.
{% endhint %}

### Related commands

{% tabs %}
{% tab title="macOS" %}
* `CMD-K` will clear the entire list of input/output blocks for a clean view
* `CTRL-L` will move the list of input/output blocks outside of the view and past the scroll so you have a clean view and the ability to easily visit past commands
* For long Blocks, you can press `SHIFT-CMD-UP`/`SHIFT-CMD-DOWN` to Scroll to the top/bottom the selected block.
{% endtab %}

{% tab title="Windows" %}
* `CTRL-SHIFT-K` will clear the entire list of input/output blocks for a clean view
* `CTRL-L` will move the list of input/output blocks outside of the view and past the scroll so you have a clear view and the ability to easily visit past commands
* For long Blocks, you can press `CTRL-SHIFT-UP`/`CTRL-SHIFT-DOWN` to Scroll to the top/bottom of the selected block.
{% endtab %}

{% tab title="Linux" %}
* `CTRL-SHIFT-K` will clear the entire list of input/output blocks for a clean view
* `CTRL-L` will move the list of input/output blocks outside of the view and past the scroll so you have a clear view and the ability to easily visit past commands
* For long Blocks, you can press `CTRL-SHIFT-UP`/`CTRL-SHIFT-DOWN` to Scroll to the top/bottom of the selected block.
{% endtab %}
{% endtabs %}

## How it works

{% embed url="https://www.youtube.com/watch?end=147&start=37&v=z1rDVPxaNCo" %}
Input Position Demo
{% endembed %}