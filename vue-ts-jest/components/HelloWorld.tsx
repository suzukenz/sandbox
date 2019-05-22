import { Vue, Component, Prop } from "vue-property-decorator";

@Component
export default class HelloWorld extends Vue {
  @Prop({ default: "TypeScript!" }) readonly name!: string;
  message: string = "Hello, ";
  render(h: Vue.CreateElement): Vue.VNode {
    return (
      <div>
        <p>
          {this.message} {this.name}
        </p>
      </div>
    );
  }
}
