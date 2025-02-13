/**
 * @vitest-environment jsdom
 */

import { describe, it } from 'vitest'

import { KVStore } from '.'

describe.concurrent('Storage', () => {
  const store = KVStore
  it('stores value to temp storage', ({ expect }) => {
    store.clearAll()

    const itemKey = 'item'
    const itemValue = { some: 'value' }
    store.setItem(itemKey, itemValue, false)

    expect(store.getItem(itemKey, false)).toEqual(itemValue)
    expect(store.getItem(itemKey, true)).null
  })

  it('stores value to persistent storage', ({ expect }) => {
    store.clearAll()

    const itemKey = 'item'
    const itemValue = { some: 'value' }
    store.setItem(itemKey, itemValue, true)

    expect(store.getItem(itemKey, true)).toEqual(itemValue)
    expect(store.getItem(itemKey, false)).null
  })

  it('removes value from single storage', ({ expect }) => {
    store.clearAll()

    const itemKey = 'item'
    const tempItemValue = { some: 'value temp' }
    const persistentItemValue = { some: 'value persistent' }

    store.setItem(itemKey, tempItemValue, false)
    store.setItem(itemKey, persistentItemValue, true)

    store.removeItem(itemKey, false)

    expect(store.getItem(itemKey, false)).null
    expect(store.getItem(itemKey, true)).toEqual(persistentItemValue)
  })

  it('clears single storage', ({ expect }) => {
    const itemKey = 'item'
    const tempItemValue = { some: 'value temp' }
    const persistentItemValue = { some: 'value persistent' }

    store.setItem(itemKey, tempItemValue, false)
    store.setItem(itemKey, persistentItemValue, true)
    store.clear(true)

    expect(store.getItem(itemKey, true)).null
    expect(store.getItem(itemKey, false)).toEqual(tempItemValue)
  })
})
