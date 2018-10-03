# Rust-Switch
Rust Switch is a simple library that provides a very simple structure that is used to wrap the boolean data type, switching it's value after it has been used.

I have provided a simple example as to where the structure can be used.

For example:

If you have a system where you need to take a boolean, and invert its value after it is used, this is the perfect implementation of this switch structure, as it does it for you.

New to rust, coming from Java? This is what it would look like in Java:
    public Switch(boolean value) {
        this.value = value;
    }

    public boolean get() {
        return !this.invert();
    }

    public boolean invert() {
        return (this.value = !this.value);
    }

    public boolean view() {
        return this.value;
    }
