---
date: '2025-10-25T13:52:48+09:00'
draft: true
title: 'Laser Shooter 개선하기'
---

동아리 프로젝트로 레이저 슈팅 프로젝트를 맡게 되었다.  

레이저 슈팅의 작동 방식은 레이저건으로 과녁을 쏘았을 때, 그 빨간 점을 감지하여 점수를 계산하는 방식.  

OpenCV로 먼저 [아루코 마커](https://en.wikipedia.org/wiki/ARTag)를 인식하여 과녁을 직사각형 형태로 변형한 뒤, 감지하는 방식이다.  

기본 작동하는 코드는 동아리 부장이 작성해서 주어졌지만, 몇가지 개선점을 찾아서 고치게 됐다.

## 레이저 포인트 감지 방식 변경

### 기존 방식

1. 이미지 색상을 HSV로 처리
2. 빨간 범위 내에 들어오는 점들을 필터링
3. 남은 부분에서 외곽선을 추출
4. 외곽선 내부 넓이가 일정 이상이면 점으로 판단

```py
# 레이저 포인트 검출 (빨강 + 밝은 점)
hsv = cv2.cvtColor(warped, cv2.COLOR_BGR2HSV)

lower_red1 = np.array([0, 150, 150])
upper_red1 = np.array([10, 255, 255])
lower_red2 = np.array([160, 150, 150])
upper_red2 = np.array([179, 255, 255])
mask1 = cv2.inRange(hsv, lower_red1, upper_red1)
mask2 = cv2.inRange(hsv, lower_red2, upper_red2)

lower_bright = np.array([0, 0, 200])
upper_bright = np.array([179, 50, 255])
mask3 = cv2.inRange(hsv, lower_bright, upper_bright)

mask = cv2.bitwise_or(cv2.bitwise_or(mask1, mask2), mask3)
mask = cv2.medianBlur(mask, 5)
```

### 변경점

기존 빨강색을 추출하는 방식은 주변 환경에 따라 인지가 달라질 수 있기 때문에, 개선점으로 이미지를 YCbCr으로 처리하는 방식을 사용하기로 했다.

```python
# BGR → YCrCb 변환 (YCbCr과 동일)
ycrcb = cv2.cvtColor(warped, cv2.COLOR_BGR2YCrCb)

# 채널 분리
Y, Cr, Cb = cv2.split(ycrcb)

# 이진 이미지
_, binary = cv2.threshold(Cr, WHITE_THRESHOLD, 255, cv2.THRESH_BINARY)
```
