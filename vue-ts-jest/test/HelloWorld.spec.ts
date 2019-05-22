import { mount } from "@vue/test-utils";
import HelloWorld from "@/components/HelloWorld.tsx";

describe("HelloWorld", () => {
  test("is a Vue instance", () => {
    const wrapper = mount(HelloWorld);
    expect(wrapper.isVueInstance()).toBeTruthy();
  });
});
