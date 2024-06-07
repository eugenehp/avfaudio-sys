# TROUBLESHOOTING

## visionOS

### 'CoreVideo/CVDisplayLink.h' file not found

`rust-bindgen` and `clang` return:

```shell
/Applications/Xcode.app/Contents/Developer/Platforms/XROS.platform/Developer/SDKs/XROS1.2.sdk/System/Library/Frameworks/CoreVideo.framework/Headers/CoreVideo.h:25:10: fatal error: 'CoreVideo/CVDisplayLink.h' file not found
/Applications/Xcode.app/Contents/Developer/Platforms/XROS.platform/Developer/SDKs/XROS1.2.sdk/System/Library/Frameworks/CoreVideo.framework/Headers/CoreVideo.h:25:10: note: did not find header 'CVDisplayLink.h' in framework 'CoreVideo' (loaded from '/Applications/Xcode.app/Contents/Developer/Platforms/XROS.platform/Developer/SDKs/XROS1.2.sdk/System/Library/Frameworks')
```

Fix: 
> [!NOTE]  
> Add `#define TARGET_OS_IPHONE 1` to your header.