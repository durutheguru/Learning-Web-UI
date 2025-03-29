<template>
  <div class="hello">
    <div class="container">
      <div
        class="
          animate-pulse
          bg-slate-200
          py-6
          flex flex-col
          relative
          overflow-hidden
          sm:py-12
        "
      >
        <div
          class="
            border
            relative
            px-4
            pt-7
            pb-8
            bg-white
            shadow-xl
            w-1/2
            max-w-md
            mx-auto
            sm:px-10
            rounded-b-md
          "
        >
          <form>
            <input
              type="text"
              id="max-price"
              class="border w-full h-10 px-3 rounded-md"
              v-model="price"
              placeholder="Maximum Price"
            />
            <button
              type="button"
              class="
                mt-5
                bg-green-500
                hover:bg-blue-700
                shadow-xl
                text-white
                uppercase
                text-sm
                font-semibold
                px-14
                py-3
                rounded
              "
              @click="updateNotification()"
            >
              Notify Me
            </button>
          </form>
        </div>
      </div>
      <br /><br />

      <!-- This is an example component -->
      <div class="max-w-2xl mx-auto">
        <div class="flex flex-col">
          <div class="overflow-x-auto shadow-md sm:rounded-lg">
            <div class="inline-block min-w-full align-middle">
              <div class="overflow-hidden">
                <table
                  class="
                    min-w-full
                    divide-y divide-gray-200
                    table-fixed
                    dark:divide-gray-700
                  "
                >
                  <thead class="bg-gray-100 dark:bg-gray-700">
                    <tr>
                      <th
                        scope="col"
                        class="
                          py-3
                          px-6
                          text-xs
                          font-medium
                          tracking-wider
                          text-left text-gray-700
                          uppercase
                          dark:text-gray-400
                        "
                      >
                        ID
                      </th>
                      <th
                        scope="col"
                        class="
                          py-3
                          px-6
                          text-xs
                          font-medium
                          tracking-wider
                          text-left text-gray-700
                          uppercase
                          dark:text-gray-400
                        "
                      >
                        Description
                      </th>
                      <th
                        scope="col"
                        class="
                          py-3
                          px-6
                          text-xs
                          font-medium
                          tracking-wider
                          text-left text-gray-700
                          uppercase
                          dark:text-gray-400
                        "
                      >
                        Price
                      </th>
                    </tr>
                  </thead>
                  <tbody
                    class="
                      bg-white
                      divide-y divide-gray-200
                      dark:bg-gray-800 dark:divide-gray-700
                    "
                  >
                    <tr
                      v-for="p in products"
                      v-bind:key="p.id"
                      class="hover:bg-gray-100 dark:hover:bg-gray-700"
                    >
                      <td
                        class="
                          py-4
                          px-6
                          text-sm
                          font-medium
                          text-gray-900
                          whitespace-nowrap
                          dark:text-white
                        "
                      >
                        {{ p.id }}
                      </td>
                      <td
                        class="
                          py-4
                          px-6
                          text-sm
                          font-medium
                          text-gray-500
                          whitespace-nowrap
                          dark:text-white
                        "
                      >
                        {{ p.description }}
                      </td>
                      <td
                        class="
                          py-4
                          px-6
                          text-sm
                          font-medium
                          text-gray-900
                          whitespace-nowrap
                          dark:text-white
                        "
                      >
                        {{ p.price }}
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

// let products = ref([])

export default defineComponent({
  name: "HelloWorld",

  props: {
    msg: {
      type: String,
      required: false,
    },
  },

  data() {
    return {
      products: [],
      price: 0,
    };
  },

  mounted() {
    console.log("mounted Hello World!!");
  },

  methods: {
    updateNotification() {
      console.log("Updating Notification");

      let self = this;
      let source = new EventSource(
        "http://localhost:8091/product/stream/" + self.price
      );
      source.onmessage = (event) => {
        console.log(`Event Data: ${JSON.stringify(event)}`);
        let product = JSON.parse(event.data);
        console.log(`Product: ${JSON.stringify(product)}`);
        (self.products as any[]).push(product);
      };
    },
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
