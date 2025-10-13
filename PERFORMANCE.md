# Performance Optimization Guide

## 🚀 Các vấn đề về hiệu suất đã được sửa

### ❌ Vấn đề 1: Component nhấp nháy (Flickering)

**Triệu chứng:**
- App bị nhấp nháy khi click button
- Hiển thị văn bản bị re-render liên tục
- UI không mượt mà khi tương tác

**Nguyên nhân:**
Component re-render không cần thiết do:
1. Function được tạo mới mỗi lần render
2. JSX render function được gọi lại mỗi lần
3. Event handlers không được memoize

**Giải pháp đã áp dụng:**

#### 1. Sử dụng `useMemo` cho nội dung được render

**Trước khi fix:**
```typescript
// ❌ Function này tạo ra JSX mới mỗi lần component render
const renderText = () => {
    const sentences = textContent.split(/[.!?]+/);
    return sentences.map((sentence, idx) => (
        <span key={idx}>...</span>
    ));
};

return <div>{renderText()}</div>; // Gọi function mỗi lần render
```

**Sau khi fix:**
```typescript
// ✅ Chỉ re-render khi textContent thay đổi
const renderedText = useMemo(() => {
    const sentences = textContent.split(/[.!?]+/);
    return sentences.map((sentence, idx) => (
        <span key={idx}>...</span>
    ));
}, [textContent, handleWordClick]);

return <div>{renderedText}</div>; // Sử dụng giá trị đã memoize
```

#### 2. Sử dụng `useCallback` cho event handlers

**Trước khi fix:**
```typescript
// ❌ Function mới được tạo mỗi lần render
const handleWordClick = async (word: string, sentence: string) => {
    // ...
};
```

**Sau khi fix:**
```typescript
// ✅ Function được memoize, không tạo mới mỗi lần render
const handleWordClick = useCallback(async (word: string, sentence: string) => {
    // ...
}, []); // Empty deps = function không bao giờ thay đổi
```

#### 3. Tối ưu dependencies trong useEffect

**Trước khi fix:**
```typescript
// ❌ loadVocabulary tạo mới mỗi render → useEffect chạy lại
const loadVocabulary = async () => { ... };

useEffect(() => {
    loadVocabulary();
}, []); // Warning: missing dependency
```

**Sau khi fix:**
```typescript
// ✅ loadVocabulary được memoize
const loadVocabulary = useCallback(async () => { ... }, []);

useEffect(() => {
    loadVocabulary();
}, [loadVocabulary]); // Dependency đúng, không trigger re-run
```

---

## 📊 So sánh hiệu suất

### Trước khi tối ưu:
- ❌ Component re-render: **Mỗi lần state thay đổi**
- ❌ Text được parse lại: **Mỗi lần render**
- ❌ Event handlers tạo mới: **Mỗi lần render**
- ❌ Effect chạy lại: **Không kiểm soát**

### Sau khi tối ưu:
- ✅ Component re-render: **Chỉ khi cần thiết**
- ✅ Text được parse lại: **Chỉ khi textContent thay đổi**
- ✅ Event handlers: **Được tái sử dụng**
- ✅ Effect: **Chỉ chạy khi dependencies thay đổi**

---

## 🎯 Best Practices

### 1. Khi nào dùng `useMemo`

✅ **NÊN dùng khi:**
- Tính toán phức tạp (parse text, filter data)
- Render danh sách lớn
- Tạo JSX elements phức tạp

❌ **KHÔNG cần dùng khi:**
- Tính toán đơn giản (a + b)
- Primitive values (string, number)
- Component nhỏ, ít re-render

**Ví dụ:**
```typescript
// ✅ Nên dùng - tính toán phức tạp
const filteredWords = useMemo(() => {
    return words.filter(w => w.length > 3)
                .sort()
                .map(w => w.toUpperCase());
}, [words]);

// ❌ Không cần - quá đơn giản
const total = useMemo(() => a + b, [a, b]); // Overkill
const total = a + b; // Đủ rồi
```

### 2. Khi nào dùng `useCallback`

✅ **NÊN dùng khi:**
- Function được pass xuống child components
- Function được dùng trong dependencies
- Event handlers phức tạp

❌ **KHÔNG cần dùng khi:**
- Function chỉ dùng local trong component
- Function không được pass đi đâu

**Ví dụ:**
```typescript
// ✅ Nên dùng - pass xuống child
const handleClick = useCallback(() => {
    doSomething();
}, []);

return <ChildComponent onClick={handleClick} />;

// ❌ Không cần - chỉ dùng local
const handleLocalClick = () => {
    setCount(count + 1);
};

return <button onClick={handleLocalClick}>Click</button>;
```

### 3. Tối ưu danh sách lớn

**Sử dụng key đúng cách:**
```typescript
// ❌ SAI - dùng index
{items.map((item, index) => (
    <div key={index}>{item}</div>
))}

// ✅ ĐÚNG - dùng unique ID
{items.map((item) => (
    <div key={item.id}>{item}</div>
))}
```

**Virtualization cho danh sách rất dài:**
```typescript
// Nếu có 1000+ items, xem xét dùng react-window hoặc react-virtual
import { FixedSizeList } from 'react-window';

<FixedSizeList
    height={600}
    itemCount={words.length}
    itemSize={100}
>
    {({ index, style }) => (
        <div style={style}>{words[index].word}</div>
    )}
</FixedSizeList>
```

### 4. Tránh inline functions trong render

**❌ SAI:**
```typescript
return (
    <button onClick={() => handleClick(id)}>
        Click
    </button>
);
```

**✅ ĐÚNG - Option 1:**
```typescript
const onClick = useCallback(() => handleClick(id), [id, handleClick]);

return <button onClick={onClick}>Click</button>;
```

**✅ ĐÚNG - Option 2 (nếu không cần memoize):**
```typescript
return <button onClick={() => handleClick(id)}>Click</button>;
// OK nếu component ít re-render
```

---

## 🔍 Debug Performance Issues

### 1. Sử dụng React DevTools Profiler

```bash
# Cài React DevTools extension cho Chrome/Firefox
# Mở DevTools → Tab "Profiler"
# Click "Record" → Tương tác với app → Click "Stop"
# Xem component nào render nhiều nhất
```

### 2. Console log để track renders

```typescript
function MyComponent() {
    console.log('MyComponent rendered');

    useEffect(() => {
        console.log('Effect ran');
    });

    // ...
}
```

### 3. Kiểm tra dependencies

```typescript
// Thêm comment để nhớ tại sao dùng dependency
const callback = useCallback(() => {
    doSomething(value);
}, [value]); // Re-create khi value thay đổi
```

---

## 📝 Checklist tối ưu performance

### Component TextReader
- ✅ Dùng `useMemo` cho `renderedText`
- ✅ Dùng `useCallback` cho `handleWordClick`
- ✅ Dùng `useCallback` cho `handleSaveWord`
- ✅ Dependencies được khai báo đúng

### Component VocabularyList
- ✅ Dùng `useCallback` cho `loadVocabulary`
- ✅ Dùng `useCallback` cho `markLearned`
- ✅ `useEffect` dependencies đúng
- ✅ Không có unnecessary re-renders

### General
- ✅ Không có inline functions trong render (khi cần thiết)
- ✅ Key props dùng unique ID
- ✅ Console logs để debug
- ✅ Không có memory leaks

---

## 🎓 Hiểu sâu hơn về React Rendering

### React re-render khi nào?

1. **State thay đổi** (`setState`)
2. **Props thay đổi**
3. **Parent component re-render** (trừ khi dùng `React.memo`)
4. **Context value thay đổi**

### Cách tránh unnecessary re-renders:

```typescript
// 1. React.memo cho component
const MyComponent = React.memo(({ data }) => {
    return <div>{data}</div>;
});

// 2. useMemo cho expensive calculations
const result = useMemo(() => expensiveCalculation(data), [data]);

// 3. useCallback cho functions
const callback = useCallback(() => doSomething(), []);

// 4. Split components
// Thay vì 1 component lớn, tách thành nhiều component nhỏ
```

---

## 🚀 Performance Tips khác

### 1. Lazy load components
```typescript
import { lazy, Suspense } from 'react';

const VocabularyList = lazy(() => import('./VocabularyList'));

<Suspense fallback={<div>Loading...</div>}>
    <VocabularyList />
</Suspense>
```

### 2. Debounce search input
```typescript
import { useMemo } from 'react';

function useDebounce(value: string, delay: number) {
    const [debouncedValue, setDebouncedValue] = useState(value);

    useEffect(() => {
        const handler = setTimeout(() => {
            setDebouncedValue(value);
        }, delay);

        return () => clearTimeout(handler);
    }, [value, delay]);

    return debouncedValue;
}
```

### 3. Batch state updates
```typescript
// ❌ 3 re-renders
setName('John');
setAge(30);
setCity('NYC');

// ✅ 1 re-render
setUser({ name: 'John', age: 30, city: 'NYC' });
```

---

## 📚 Tài liệu tham khảo

- [React Performance Optimization](https://react.dev/learn/render-and-commit)
- [useMemo](https://react.dev/reference/react/useMemo)
- [useCallback](https://react.dev/reference/react/useCallback)
- [React.memo](https://react.dev/reference/react/memo)
