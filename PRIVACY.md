# Privacy

VibeCut is a local video editor. Your footage, your audio and your images never
leave your computer. This page lists every time the app touches the network, so
you can check the claim instead of trusting it.

| What leaves your machine | When | What it contains |
|---|---|---|
| AI model downloads | only if you download one | the request for the file. Nothing about you, your projects or your footage |
| Version check | when you open Settings › Version, or press *Check for updates* | nothing about you. It asks GitHub for the list of releases |
| **AI chat** | **only when you open the chat and send a message** | see below |

## The AI chat, in detail

The chat is off unless you configure a provider and open the panel.

**What it sends**, to the provider you chose and to no one else:

- the message you typed, and the conversation so far
- the tool definitions the assistant may call
- a summary of your timeline: clip names, times, tracks, and the ids of the
  effects, titles and transitions in use

**What it never sends:**

- video, audio or image data — not a frame of it
- the files on your disk, or their paths
- your projects' folder structure

The summary is what lets the assistant answer "make the intro two seconds
longer" without seeing the video. If you would rather it saw nothing, close the
chat: with the panel closed, no request is made.

## Where your API key lives

Keys are stored encrypted with Windows DPAPI, bound to your Windows account, in
`%APPDATA%\pro.mercadodigital.vibecut\`. They are never written to a project
file, never sent anywhere except to that provider, and deleted when you remove
the provider.

## What VibeCut does not do

- no account, no sign-in, no telemetry, no analytics, no crash reports sent
  automatically (the crash dialog offers you the text to copy)
- no background requests while the chat is closed
- no model is downloaded without you pressing the button

## Questions

Open an issue at <https://github.com/OscarGallardo35/VibeCut/issues>.
