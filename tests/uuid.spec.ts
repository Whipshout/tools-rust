// Rust binary import
import { uuid } from "../index";

// We have to use 'ava' to test 'napi'
// Ava load a node environment needed for Rust binary
import test from 'ava'

import { randomBytes } from "crypto";

test('should return a valid random uuid', (t) => {
    // Input
    const id = uuid()

    // Assertions
    t.is(typeof id, 'string')
    t.not(id, '')
    t.is(id.length, 36)
    t.regex(id, /-/)
})

test('should return a valid uuid using an input', (t) => {
    // Generates random 36 alphanumeric characters string
    const data = randomBytes(18).toString('hex')

    // Input
    const id = uuid(data)

    // Assertions
    t.is(typeof id, 'string')
    t.not(id, '')
    t.is(id.length, 36)
    t.regex(id, /-/)
})
