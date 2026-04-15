<script setup lang="ts">
import HelloWorld from './components/HelloWorld.vue'
import { Button as VarButton } from '@varlet/ui'
import '@varlet/ui/es/button/style/index'
</script>

<template>
  <HelloWorld />
  <var-button type="primary">Example</var-button>
</template>
